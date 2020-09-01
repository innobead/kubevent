use clap::App;

pub const CMD_NAME: &str = "rulebinding";

pub fn new() -> App<'static> {
    App::new(CMD_NAME).about("manage rules-broker bindings")
}
