use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct TestPlugin {}

impl Plugin for TestPlugin {
    fn name(&self) -> &str {
        &"test_plugin"
    }

    fn description(&self) -> &str {
        &"Test plugin"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        Err("Method not implemented".to_string())
    }

    fn process(&self, _output: &str) -> Result<Value, String> {
        Err("Method not implemented".to_string())
    }
}

impl TestPlugin {
    pub fn new() -> Self {
        TestPlugin {}
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[test]
    fn test_split_list() {
        let command_output = "
item1
item2
item3
";
        let parsed_data = json!(["item1", "item2", "item3"]);
        let test_plugin = TestPlugin::new();
        assert_eq!(parsed_data, test_plugin._split_list(command_output).unwrap());
    }

    #[test]
    fn test_convert_json_string() {
        let command_output = r#"
[
    {
        "head1": "val11",
        "head2": "val12",
        "head3": "val13"
    },
    {
        "head1": "val21",
        "head2": "val22",
        "head3": "val23"
    },
    {
        "head1": "val31",
        "head2": "val32",
        "head3": "val33"
    }
]
"#;
        let parsed_data = json!([
            {
                "head1": "val11",
                "head2": "val12",
                "head3": "val13"
            },
            {
                "head1": "val21",
                "head2": "val22",
                "head3": "val23"
            },
            {
                "head1": "val31",
                "head2": "val32",
                "head3": "val33"
            }
        ]);
        let test_plugin = TestPlugin::new();
        assert_eq!(parsed_data, test_plugin._convert_json_string(command_output).unwrap());
    }

    #[test]
    fn test_convert_csv_string_no_header() {
        let command_output = "
val11 val12 val13
val21 val22 val23
val31 val32 val33
";
        let parsed_data = json!([
            {
                "head1": "val11",
                "head2": "val12",
                "head3": "val13"
            },
            {
                "head1": "val21",
                "head2": "val22",
                "head3": "val23"
            },
            {
                "head1": "val31",
                "head2": "val32",
                "head3": "val33"
            }
        ]);
        let test_plugin = TestPlugin::new();
        assert_eq!(parsed_data, test_plugin._convert_csv_string_no_header(command_output, &vec!["head1", "head2", "head3"], &" ").unwrap());
    }

    #[test]
    fn test_convert_csv_string_with_header() {
        let command_output = "
head1 head2 head3
val11 val12 val13
val21 val22 val23
val31 val32 val33
";
        let parsed_data = json!([
            {
                "head1": "val11",
                "head2": "val12",
                "head3": "val13"
            },
            {
                "head1": "val21",
                "head2": "val22",
                "head3": "val23"
            },
            {
                "head1": "val31",
                "head2": "val32",
                "head3": "val33"
            }
        ]);
        let test_plugin = TestPlugin::new();
        assert_eq!(parsed_data, test_plugin._convert_csv_string_with_header(command_output, &" ").unwrap());
    }
}