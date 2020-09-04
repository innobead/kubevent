#![allow(dead_code)] // avoid incorrect warning about dead_code

use anyhow::Result;
use cloudevents::Event;
use kubevent_common::crd::BrokerSpec;

pub mod console;

pub trait BrokerTrait {
    fn send(&self, spec: BrokerSpec, event: Event) -> Result<()>;
}
