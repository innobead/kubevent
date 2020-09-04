use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use anyhow::Result;
use futures::StreamExt;
use kube::api::{ListParams, Meta};
use kube::{Api, Client};
use kube_runtime::controller::{Context, ReconcilerAction};
use kube_runtime::Controller;
use serde::de::DeserializeOwned;
use snafu::Snafu;
use tokio::sync::Mutex;
use tokio::time::Duration;

use crate::controllers::ResourceExtTrait;
use crate::crd;

type RuleName = String;
type BrokerName = String;

pub struct ResourceController;

pub struct ResourceProcessor {
    pub rules: HashMap<RuleName, crd::RuleSpec>,
    pub brokers: HashMap<BrokerName, crd::BrokerSpec>,
    pub rule_bindings: HashMap<BrokerName, Vec<RuleName>>,
}

struct ReconcileData {
    client: Client,
    processor: Arc<Mutex<ResourceProcessor>>,
}

#[derive(Debug, Snafu)]
enum ReconcileError {}

impl ResourceController {
    pub fn new() -> Self {
        ResourceController
    }

    pub async fn start<T>(
        &self,
        resource_type: &str,
        resource_processor: Arc<Mutex<ResourceProcessor>>, /*Arc<ResourceProcessor>*/
    ) -> Result<()>
    where
        T: kube::api::Meta + Debug + Clone + Send + Sync + DeserializeOwned + 'static,
    {
        log::info!("starting {} controller", resource_type);

        let client = Client::try_default().await?;
        let api = Api::<T>::all(client.clone());

        Controller::new(api.clone(), ListParams::default())
            .run(
                reconcile,
                error_policy,
                Context::new(ReconcileData {
                    client,
                    processor: Arc::clone(&resource_processor),
                }),
            )
            .for_each(|res| async {
                match res {
                    Err(err) => {
                        log::error!("failed to reconcile resource: {:?}", err);
                    }

                    _ => {}
                }
            })
            .await;

        Ok(())
    }
}

impl ResourceProcessor {
    pub fn new() -> Self {
        ResourceProcessor {
            rules: Default::default(),
            brokers: Default::default(),
            rule_bindings: Default::default(),
        }
    }

    fn process(&mut self, res: &dyn Any) -> Result<()> {
        // rule
        if TypeId::of::<crd::Rule>() == res.type_id() {
            if let Some(value) = res.downcast_ref::<crd::Rule>() {
                log::info!("processing resource: {}", value.description());

                let name = Meta::name(value);
                let value = &value.spec as &crd::RuleSpec;

                self.rules.insert(name, value.clone());
            }

            return Ok(());
        }

        // broker
        if TypeId::of::<crd::Broker>() == res.type_id() {
            if let Some(value) = res.downcast_ref::<crd::Broker>() {
                log::info!("processing resource: {}", value.description());

                let name = Meta::name(value);
                let value = &value.spec as &crd::BrokerSpec;

                self.brokers.insert(name, value.clone());
            }

            return Ok(());
        }

        // rule binding
        if TypeId::of::<crd::RuleBinding>() == res.type_id() {
            if let Some(value) = res.downcast_ref::<crd::RuleBinding>() {
                log::info!("processing resource: {}", value.description());

                let rule_binding_spec = &value.spec as &crd::RuleBindingSpec;

                if !self.rules.contains_key(rule_binding_spec.rule.as_str()) {
                    return Err(anyhow::Error::msg(format!(
                        "rule not found: {}",
                        rule_binding_spec.rule
                    )));
                }

                for broker in &rule_binding_spec.brokers {
                    if !self.rule_bindings.contains_key(broker.as_str()) {
                        self.rule_bindings
                            .insert(broker.clone(), Default::default());
                    }

                    let binding_rules = self.rule_bindings.get_mut(broker.as_str()).unwrap();

                    if !binding_rules.contains(&rule_binding_spec.rule) {
                        binding_rules.push(rule_binding_spec.rule.clone());
                    }
                }
            }
        }

        return Ok(());
    }

    fn sync_resource_meta_finalizer(&mut self, _res: &dyn Any) -> Result<()> {
        // TODO add/delete the finalizer of resource
        // if TypeId::of::<crd::Rule>() == res.type_id() {
        //     Api::<crd::Rule>::all().patch()
        // }

        Ok(())
    }
}

async fn reconcile<T>(
    resource: T,
    ctx: Context<ReconcileData>,
) -> Result<ReconcilerAction, ReconcileError>
where
    T: kube::api::Meta + Debug + 'static,
{
    log::info!("reconciling resource: {}", resource.description());

    let processor = &ctx.get_ref().processor;

    let mut processor = processor.lock().await;

    return match processor.process(&resource) {
        Ok(_) => Ok(ReconcilerAction {
            requeue_after: None,
        }),

        Err(err) => {
            log::error!("failed to reconcile, requeue the resource: {:?}", err);

            Ok(ReconcilerAction {
                requeue_after: Some(Duration::from_secs(1)),
            })
        }
    };
}

fn error_policy<E>(_: &E, _ctx: Context<ReconcileData>) -> ReconcilerAction
where
    E: std::error::Error,
{
    ReconcilerAction {
        requeue_after: Some(Duration::from_secs(1)),
    }
}
