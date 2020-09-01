use clap::App;

pub const CMD_NAME: &str = "uninstall";

pub fn new() -> App<'static> {
    App::new(CMD_NAME).about("uninstall kubeventd")
}
