use clap::App;

pub const CMD_NAME: &str = "rulebrokersbinding";

pub fn new<'help>() -> App<'help> {
    App::new(CMD_NAME).about("manage rules-broker bindings")
}
