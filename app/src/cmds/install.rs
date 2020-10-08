use clap::App;

pub const CMD_NAME: &str = "install";

pub fn new<'help>() -> App<'help> {
    App::new(CMD_NAME).about("install kubeventd")
}
