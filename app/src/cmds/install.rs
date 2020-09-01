use clap::App;

pub const CMD_NAME: &str = "install";

pub fn new() -> App<'static> {
    App::new(CMD_NAME).about("install kubeventd")
}
