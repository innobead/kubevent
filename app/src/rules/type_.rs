use std::any::type_name;

use anyhow::Result;
use k8s_openapi::api::core::v1::Event;

use kubevent_common::crd::RuleSpec;

use crate::rules::RuleTrait;

#[derive(Debug)]
pub struct TypeRule;

impl TypeRule {
    pub fn new() -> Self {
        TypeRule
    }
}

impl RuleTrait for TypeRule {
    fn process(
        &self,
        spec: RuleSpec,
        raw_event: &Event,
        event: &cloudevents::Event,
    ) -> Result<cloudevents::Event> {
        let self_name = type_name::<TypeRule>();
        log::info!("applying rule: {:?}", self_name);

        if let Some(type_) = raw_event.type_.clone() {
            if spec.types.unwrap_or_default().contains(&type_) {
                return Ok(event.clone());
            }

            return Err(anyhow::Error::msg(format!(
                "ignored event because it failed to pass the rule: {:?} type={}",
                self_name, type_
            )));
        }

        Err(anyhow::Error::msg(
            "ignored event because it failed to pass the rule: the event has no type attribute",
        ))
    }
}
