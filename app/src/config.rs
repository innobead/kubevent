use std::path::PathBuf;

use env_logger::WriteStyle;

pub trait ConfigTrait {
    fn init(&self);
    fn default() -> Self;
}

pub struct Config {
    pub kube_config: PathBuf,
    pub log_level: log::Level,
    pub log_color: bool,
}

pub struct KubeventDConfig {
    base: Config,
    pub addr: &'static str,
}

impl ConfigTrait for Config {
    fn init(&self) {
        env_logger::builder()
            .write_style(if self.log_color {
                WriteStyle::Auto
            } else {
                WriteStyle::Never
            })
            .filter_level(self.log_level.to_level_filter())
            .init()
    }

    fn default() -> Self {
        Self {
            kube_config: dirs::home_dir().unwrap().join(".kube").join("config"),
            log_level: log::Level::Info,
            log_color: true,
        }
    }
}

impl ConfigTrait for KubeventDConfig {
    fn init(&self) {
        self.base.init()
    }

    fn default() -> Self {
        Self {
            base: Config::default(),
            addr: "localhost:8899",
        }
    }
}
