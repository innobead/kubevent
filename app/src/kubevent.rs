use clap::{crate_name, crate_version, App, Arg, ArgMatches};

use config::ConfigTrait;

mod cmds;
mod config;

const ARG_LOG_LEVEL: &str = "log-level";
const ARG_NO_LOG_COLOR: &str = "no-log-color";
const ARG_KUBE_CONFIG: &str = "kubeconfig";

fn main() {
    let long_version = get_long_version();

    let mut app = App::new(crate_name!())
        .about("Manages Kubernetes cloud events telemetry")
        .long_version(long_version.as_str())
        .args(&[
            Arg::with_name(ARG_LOG_LEVEL)
                .about("Log level")
                .long(ARG_LOG_LEVEL)
                .takes_value(true)
                .possible_values(&["ERROR", "WARN", "INFO", "DEBUG", "TRACE"]),
            Arg::with_name(ARG_NO_LOG_COLOR)
                .about("Disable log color")
                .long(ARG_NO_LOG_COLOR),
            Arg::with_name(ARG_KUBE_CONFIG)
                .about("Path to the kubeconfig file")
                .long(ARG_KUBE_CONFIG)
                .takes_value(true),
        ])
        .subcommands(vec![
            cmds::install::new(),
            cmds::uninstall::new(),
            cmds::broker::new(),
            cmds::rule::new(),
            cmds::rule_binding::new(),
            cmds::status::new(),
        ]);

    let matches = app.get_matches_mut();
    let config = process_args(&matches);

    config.init();

    process_cmds(&mut app, &config, &matches);
}

fn get_long_version() -> String {
    format!(
        "
version: {version}
build: {build}
",
        version = crate_version!(),
        build = "",
    )
}

fn process_args(matches: &ArgMatches) -> config::Config {
    let mut config = config::Config::default();

    if matches.is_present(ARG_LOG_LEVEL) {
        if let Ok(log_level) = matches.value_of_t::<log::Level>(ARG_LOG_LEVEL) {
            config.log_level = log_level
        }
    }

    if matches.is_present(ARG_NO_LOG_COLOR) {
        config.log_color = false
    }

    if matches.is_present(ARG_KUBE_CONFIG) {}

    config
}

fn process_cmds(app: &mut App, _config: &config::Config, matches: &ArgMatches) {
    match matches.subcommand() {
        (cmds::install::CMD_NAME, Some(_subcommand_matches)) => unimplemented!(),

        (cmds::uninstall::CMD_NAME, Some(_subcommand_matches)) => unimplemented!(),

        (cmds::broker::CMD_NAME, Some(_subcommand_matches)) => unimplemented!(),

        (cmds::rule::CMD_NAME, Some(_subcommand_matches)) => unimplemented!(),

        (cmds::rule_binding::CMD_NAME, Some(_subcommand_matches)) => unimplemented!(),

        (cmds::status::CMD_NAME, Some(_subcommand_matches)) => unimplemented!(),

        _ => {
            let _ = app.print_help();
        }
    }
}
