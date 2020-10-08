use clap::App;

pub const CMD_NAME: &str = "uninstall";

pub fn new<'help>() -> App<'help> {
    App::new(CMD_NAME).about("uninstall kubeventd")
}
