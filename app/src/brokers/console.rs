use anyhow::Result;
use cloudevents::Event;

use kubevent_common::crd::BrokerSpec;

use crate::brokers::BrokerTrait;

pub struct ConsoleBroker;

impl ConsoleBroker {
    pub fn new() -> Self {
        ConsoleBroker
    }
}

impl BrokerTrait for ConsoleBroker {
    fn send(&self, _: BrokerSpec, event: Event) -> Result<()> {
        log::info!("Sending {:#?}", event);

        Ok(())
    }
}
