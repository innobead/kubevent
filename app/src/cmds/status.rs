use clap::App;

pub const CMD_NAME: &str = "status";

pub fn new<'help>() -> App<'help> {
    App::new(CMD_NAME).about("show status of resources")
}
