
use configparser::ini::Ini;

pub struct Config {
    pub config: Ini,
}

impl Config {
    pub fn new(config_file: &str) -> Result<Config, String> {
        let mut config = Ini::new();
        match config.load(config_file) {
            Ok(_) => Ok(Config { config }),
            Err(e) => Err(e),
        } 
    }

    pub fn is_plugin_enabled(&self, plugin_name: &str) -> bool {
        // The default plugin status is disabled
        match self.config.getbool(plugin_name, &"enabled") {
            Ok(v) => match v {
                Some(v) => v,
                _ => false,
            },
            Err(_) => false,
        }
    }

    pub fn get_boolean_setting(&self, plugin_name: &str, setting_name: &str) -> bool {
        // The default bool value is "false"
        match self.config.getbool(plugin_name, setting_name) {
            Ok(v) => match v {
                Some(v) => v,
                _ => false,
            },
            Err(_) => false,
        }
    }

    pub fn get_string_setting(&self, plugin_name: &str, setting_name: &str) -> Option<String> {
        // The default string value is delegated to the plugins
        self.config.get(plugin_name, setting_name)
    }
}