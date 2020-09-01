mod config;
mod controllers;

use config::ConfigTrait;
use log::info;

fn main() {
    config::Config::default().init();

    shutdown_hooks();

    // start metrics server
    start_metrics_server();

    // start controllers
    start_controllers();
}

fn shutdown_hooks() {
    info!("enabling shutdown hooks")
}

fn start_metrics_server() {
    info!("starting metrics server")
}

fn start_controllers() {
    info!("starting controllers")
}
