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
mod server;
mod watchers;

use crate::config::KubeventDConfig;
use actix_web::{web, App, HttpServer};
use actix_web_prom::PrometheusMetrics;

#[tokio::main]
async fn main() {
    let config = init();

    let resource_processor = Arc::new(Mutex::new(ResourceProcessor::new()));

    let (result_start_metrics_server, result_start_event_watcher, result_start_controllers) = tokio::join!(
        start_metrics_server(&config),
        start_event_watcher(&config, &resource_processor),
        start_controllers(&config, &resource_processor),
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

fn init() -> KubeventDConfig {
    let config = config::KubeventDConfig::default();
    config.init();

    config
}

async fn start_metrics_server(config: &config::KubeventDConfig) -> futures::io::Result<()> {
    let local = tokio::task::LocalSet::new();
    let system = actix_rt::System::run_in_tokio("server", &local);
    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);

    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .service(web::resource("/healthz").to(server::http::health))
    })
    .bind(config.addr)?
    .run()
    .await?;

    system.await
}

async fn start_event_watcher(
    config: &config::KubeventDConfig,
    resource_processor: &Arc<Mutex<ResourceProcessor>>,
) -> Result<()> {
    log::info!("starting event watcher");

    let mut brokers = HashMap::<InstanceName, Box<dyn BrokerTrait>>::new();
    brokers.insert("console".to_string(), Box::new(ConsoleBroker::new()));

    let mut rules = HashMap::<InstanceName, Box<dyn RuleTrait>>::new();
    rules.insert("type".to_string(), Box::new(TypeRule::new()));

    EventWatcher::new(brokers, rules, resource_processor.clone())
        .start()
        .await
}

async fn start_controllers(
    config: &config::KubeventDConfig,
    resource_processor: &Arc<Mutex<ResourceProcessor>>,
) -> Result<()> {
    log::info!("starting controllers");

    let controller = ResourceController::new();

    let (_, _, _) = tokio::join!(
        controller.start::<crd::Rule>("rule", resource_processor.clone()),
        controller.start::<crd::Broker>("broker", resource_processor.clone()),
        controller
            .start::<crd::RuleBrokersBinding>("rule-brokers-binding", resource_processor.clone()),
    );

    Ok(())
}
