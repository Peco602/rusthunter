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
        match self.config.getbool(plugin_name, "enabled") {
            Ok(v) => v.unwrap_or_default(),
            Err(_) => false,
        }
    }

    pub fn get_boolean_setting(&self, plugin_name: &str, setting_name: &str) -> bool {
        // The default bool value is "false"
        match self.config.getbool(plugin_name, setting_name) {
            Ok(v) => v.unwrap_or_default(),
            Err(_) => false,
        }
    }

    pub fn get_string_setting(&self, plugin_name: &str, setting_name: &str) -> Option<String> {
        // The default string value is delegated to the plugins
        self.config.get(plugin_name, setting_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_config(content: &str) -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.ini");
        let mut file = File::create(&config_path).unwrap();
        write!(file, "{}", content).unwrap();
        (temp_dir, config_path.to_str().unwrap().to_string())
    }

    #[test]
    fn test_config_new_valid() {
        let (_temp_dir, config_path) = create_test_config("[test_plugin]\nenabled=true\n");
        let config = Config::new(&config_path);
        assert!(config.is_ok());
    }

    #[test]
    fn test_config_new_nonexistent() {
        let config = Config::new("/nonexistent/path/config.ini");
        assert!(config.is_err());
    }

    #[test]
    fn test_is_plugin_enabled_true() {
        let (_temp_dir, config_path) = create_test_config("[test_plugin]\nenabled=true\n");
        let config = Config::new(&config_path).unwrap();
        assert!(config.is_plugin_enabled("test_plugin"));
    }

    #[test]
    fn test_is_plugin_enabled_false() {
        let (_temp_dir, config_path) = create_test_config("[test_plugin]\nenabled=false\n");
        let config = Config::new(&config_path).unwrap();
        assert!(!config.is_plugin_enabled("test_plugin"));
    }

    #[test]
    fn test_is_plugin_enabled_missing() {
        let (_temp_dir, config_path) = create_test_config("[test_plugin]\n");
        let config = Config::new(&config_path).unwrap();
        assert!(!config.is_plugin_enabled("test_plugin"));
    }

    #[test]
    fn test_is_plugin_enabled_nonexistent_plugin() {
        let (_temp_dir, config_path) = create_test_config("[test_plugin]\nenabled=true\n");
        let config = Config::new(&config_path).unwrap();
        assert!(!config.is_plugin_enabled("nonexistent_plugin"));
    }

    #[test]
    fn test_get_boolean_setting_true() {
        let (_temp_dir, config_path) = create_test_config("[test_plugin]\nsome_flag=true\n");
        let config = Config::new(&config_path).unwrap();
        assert!(config.get_boolean_setting("test_plugin", "some_flag"));
    }

    #[test]
    fn test_get_boolean_setting_false() {
        let (_temp_dir, config_path) = create_test_config("[test_plugin]\nsome_flag=false\n");
        let config = Config::new(&config_path).unwrap();
        assert!(!config.get_boolean_setting("test_plugin", "some_flag"));
    }

    #[test]
    fn test_get_boolean_setting_missing() {
        let (_temp_dir, config_path) = create_test_config("[test_plugin]\n");
        let config = Config::new(&config_path).unwrap();
        assert!(!config.get_boolean_setting("test_plugin", "nonexistent"));
    }

    #[test]
    fn test_get_string_setting_present() {
        let (_temp_dir, config_path) = create_test_config("[test_plugin]\npath=/usr/bin\n");
        let config = Config::new(&config_path).unwrap();
        assert_eq!(
            config.get_string_setting("test_plugin", "path"),
            Some("/usr/bin".to_string())
        );
    }

    #[test]
    fn test_get_string_setting_missing() {
        let (_temp_dir, config_path) = create_test_config("[test_plugin]\n");
        let config = Config::new(&config_path).unwrap();
        assert_eq!(
            config.get_string_setting("test_plugin", "nonexistent"),
            None
        );
    }

    #[test]
    fn test_get_string_setting_empty() {
        let (_temp_dir, config_path) = create_test_config("[test_plugin]\npath=\n");
        let config = Config::new(&config_path).unwrap();
        let result = config.get_string_setting("test_plugin", "path");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "");
    }
}
