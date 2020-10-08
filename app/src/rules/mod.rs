use anyhow::Result;
use cloudevents::event::Data;
use k8s_openapi::api::core::v1::Event;

use kubevent_common::crd::RuleSpec;

pub mod cloud_event;
pub mod type_;

pub trait RuleTrait {
    fn process(
        &self,
        spec: RuleSpec,
        raw_event: &Event,
        cloud_event: &cloudevents::Event,
    ) -> Result<cloudevents::Event>;
}

fn create_cloud_event_data(event: &Event) -> Data {
    if let Ok(value) = serde_json::to_value(event) {
        return Data::Json(value);
    }

    Data::String(String::new())
}
