use clap::App;

pub const CMD_NAME: &str = "broker";

pub fn new<'help>() -> App<'help> {
    App::new(CMD_NAME).about("manage brokers")
}
