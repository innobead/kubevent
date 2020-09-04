use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;

use config::ConfigTrait;
use kubevent_common::crd;

use crate::brokers::console::ConsoleBroker;
use crate::brokers::BrokerTrait;
use crate::controllers::resource::{ResourceController, ResourceProcessor};
use crate::rules::type_::TypeRule;
use crate::rules::RuleTrait;
use crate::watchers::event::{EventWatcher, InstanceName};

mod brokers;
mod config;
mod controllers;
mod rules;
mod watchers;

#[tokio::main]
async fn main() {
    init();

    let resource_processor = Arc::new(Mutex::new(ResourceProcessor::new()));

    let (result_start_metrics_server, result_start_event_watcher, result_start_controllers) = tokio::join!(
        start_metrics_server(),
        start_event_watcher(&resource_processor),
        start_controllers(&resource_processor),
    );

    if let Err(err) = result_start_metrics_server {
        log::error!("failed to start metrics server: {}", err)
    }

    if let Err(err) = result_start_event_watcher {
        log::error!("failed to start event watcher: {}", err)
    }

    if let Err(err) = result_start_controllers {
        log::error!("failed to start controllers: {}", err)
    }
}

fn init() {
    config::Config::default().init();
}

async fn start_metrics_server() -> Result<()> {
    Ok(())
}

async fn start_event_watcher(resource_processor: &Arc<Mutex<ResourceProcessor>>) -> Result<()> {
    log::info!("starting event watcher");

    let mut brokers = HashMap::<InstanceName, Box<dyn BrokerTrait>>::new();
    brokers.insert("console".to_string(), Box::new(ConsoleBroker::new()));

    let mut rules = HashMap::<InstanceName, Box<dyn RuleTrait>>::new();
    rules.insert("type".to_string(), Box::new(TypeRule::new()));

    EventWatcher::new(brokers, rules, resource_processor.clone())
        .start()
        .await
}

async fn start_controllers(resource_processor: &Arc<Mutex<ResourceProcessor>>) -> Result<()> {
    log::info!("starting controllers");

    let controller = ResourceController::new();

    let (_, _, _) = tokio::join!(
        controller.start::<crd::Rule>("rule", resource_processor.clone()),
        controller.start::<crd::Broker>("broker", resource_processor.clone()),
        controller.start::<crd::RuleBinding>("rule-binding", resource_processor.clone()),
    );

    Ok(())
}
