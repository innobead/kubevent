use clap::App;

pub const CMD_NAME: &str = "rule";

pub fn new<'help>() -> App<'help> {
    App::new(CMD_NAME).about("manage rules")
}
