use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::Event;
use kube::api::ListParams;
use kube::{Api, Client};
use kube_runtime::{utils::try_flatten_applied, watcher};
use tokio::sync::Mutex;

use crate::brokers::BrokerTrait;
use crate::controllers::resource::ResourceProcessor;
use crate::rules::cloud_event::CloudEventRule;
use crate::rules::RuleTrait;

pub type InstanceName = String;

pub struct EventWatcher {
    brokers: HashMap<InstanceName, Box<dyn BrokerTrait>>,
    rules: HashMap<InstanceName, Box<dyn RuleTrait>>,
    resource_processor: Arc<Mutex<ResourceProcessor>>,
}

impl EventWatcher {
    pub fn new(
        brokers: HashMap<InstanceName, Box<dyn BrokerTrait>>,
        rules: HashMap<InstanceName, Box<dyn RuleTrait>>,
        resource_processor: Arc<Mutex<ResourceProcessor>>,
    ) -> Self {
        EventWatcher {
            brokers,
            rules,
            resource_processor,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        log::info!("starting event watcher");

        let client = Client::try_default().await?;
        let api: Api<Event> = Api::all(client);
        let list_params = ListParams::default();
        let mut event_stream = try_flatten_applied(watcher(api, list_params)).boxed();

        while let Ok(event) = event_stream.try_next().await {
            if let Some(event) = event {
                if let Err(err) = self.handle_event(event).await {
                    log::warn!("{:?}", err)
                }
            }
        }

        Ok(())
    }

    async fn handle_event(&mut self, event: Event) -> Result<()> {
        log::info!(
            "handling event: {:?} {:?} {:?}",
            event.type_.clone().unwrap(),
            event.reason.clone().unwrap(),
            event.message.clone().unwrap()
        );

        let processor = self.resource_processor.lock().await;
        let cloud_event_rule = CloudEventRule::new();

        'next_broker: for (broker_name, binding) in &processor.rule_bindings {
            let broker_spec = match processor.brokers.get(broker_name.as_str()) {
                Some(s) => s,
                _ => {
                    return Err(anyhow::Error::msg(format!(
                        "broker ({}) is not found, maybe not reconciled yet",
                        broker_name
                    )));
                }
            };

            let broker_instance_name = broker_spec.kind.clone();

            if !self.brokers.contains_key(broker_instance_name.as_str()) {
                continue;
            }

            let mut cloud_event = cloudevents::Event::default();
            // builtin rule executed first
            cloud_event = cloud_event_rule
                .process(Default::default(), &event, &cloud_event)
                .unwrap();

            for rule_name in binding {
                let rule_spec = processor.rules.get(rule_name).unwrap();
                let rule_instance_name = rule_spec.kind.clone();

                if let Some(rule) = self.rules.get(rule_instance_name.as_str()) {
                    match rule.process(rule_spec.clone(), &event, &cloud_event) {
                        Ok(value) => cloud_event = value,
                        Err(err) => {
                            log::info!("{:?}", err);
                            continue 'next_broker;
                        }
                    }
                }
            }

            let broker = self.brokers.get(broker_instance_name.as_str()).unwrap();

            if let Err(e) = broker.send(broker_spec.clone(), cloud_event) {
                log::error!("failed to send event to {}: {:?}", broker_name, e);
            }
        }

        Ok(())
    }
}
