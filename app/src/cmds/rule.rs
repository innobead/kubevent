use clap::App;

pub const CMD_NAME: &str = "rule";

pub fn new() -> App<'static> {
    App::new(CMD_NAME).about("manage rules")
}
