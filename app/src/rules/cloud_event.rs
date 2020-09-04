use anyhow::Result;
use cloudevents::{EventBuilder, EventBuilderV10};
use k8s_openapi::api::core::v1::Event;
use k8s_openapi::chrono::{DateTime, Utc};
use kube::api::Meta;
use std::any::type_name;

use crate::rules::{create_cloud_event_data, RuleTrait};
use kubevent_common::crd::RuleSpec;

#[derive(Debug)]
pub struct CloudEventRule;

impl CloudEventRule {
    pub fn new() -> Self {
        CloudEventRule
    }
}

impl RuleTrait for CloudEventRule {
    fn process(
        &self,
        _: RuleSpec,
        raw_event: &Event,
        _: &cloudevents::Event,
    ) -> Result<cloudevents::Event> {
        log::info!("applying rule: {:?}", type_name::<CloudEventRule>());

        let data = create_cloud_event_data(&raw_event);
        let raw_event = raw_event.clone();

        let mut builder = EventBuilderV10::new()
            .id(raw_event.meta().uid.as_ref().unwrap())
            .source(source(&raw_event))
            .ty(raw_event.type_.as_ref().unwrap())
            .subject(subject(&raw_event))
            .data("application/json", data);

        if let Some(t) = time(&raw_event) {
            builder = builder.time(t);
        }

        match builder.build() {
            Ok(value) => Ok(value),
            Err(err) => Err(anyhow::Error::new(err)),
        }
    }
}

fn source(event: &Event) -> String {
    format!(
        "k8s://{}/{}/{}/{}/{}",
        event
            .source
            .as_ref()
            .unwrap()
            .host
            .as_ref()
            .unwrap_or(&"none".to_string()),
        event
            .source
            .as_ref()
            .unwrap()
            .component
            .as_ref()
            .unwrap_or(&"none".to_string()),
        event
            .involved_object
            .namespace
            .as_ref()
            .unwrap_or(&"none".to_string()),
        event
            .involved_object
            .kind
            .as_ref()
            .unwrap_or(&"none".to_string()),
        event
            .involved_object
            .name
            .as_ref()
            .unwrap_or(&"none".to_string()),
    )
}

fn time(event: &Event) -> Option<DateTime<Utc>> {
    Some(event.last_timestamp.as_ref()?.0)
}

fn subject(event: &Event) -> String {
    format!(
        "{} ### {}",
        event.reason.as_ref().unwrap(),
        event.message.as_ref().unwrap(),
    )
}
