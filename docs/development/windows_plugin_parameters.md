# How to develop a Windows plugin with parameters

In this tutorial it will be shown how to develop a plugin that collects the members of a group in a Windows Active Directory domain. It can be considered as a guideline to create additional plugins for Windows.


### 1. Scaffolding

The first steps consists of creating the structure of the plugin based on the sample plugin already made available. The plugin name will be `windows_domain_group` so the command to be executed is the following:

```console
user@master-node:~/rusthunter$ cp -r app/src/plugins/sample app/src/plugins/windows/domain_group
```

The plugin folder contains the `mod.rs` file which provides the plugin logic:

```console
user@master-node:~/rusthunter$ ls -al app/src/plugins/windows/domain_group
mod.rs
```

### 2. Customization

The file `app/src/plugins/windows/domain_group/mod.rs` must be properly customized to collect the desired data on the target machine.

1. Replace all the occurrencies of the struct name `SamplePlugin` with `WindowsDomainGroup`. It can be easily done via the command:

```console
user@master-node:~/rusthunter$ sed -i 's/SamplePlugin/WindowsDomainGroup/g' app/src/plugins/windows/domain_group/mod.rs
```

2. Replace the plugin name `sample_plugin` with `windows_domain_group` (Line 10).

3. Replace the plugin description `Sample description` with `Domain users and groups` (Line 14).

4. Replace the plugin operating system `OS::Unknown` with `OS::Windows` (Line 18).

5. Replace the plugin line `let command = "sample command";` (Line 22) with the following snippet:

    ```rust
    let group_name =  match _config.get_string_setting(self.name(), &"group_name") {
        Some(v) => { if validate_windows_sam_account_name(&v) {
                v
            } else {
                return Err(format!("Not valid group_name setting: {}", v));
            }}, 
        None => "Domain Admins".to_string(),
    };
    let command = format!("Get-ADGroupMember -Identity \"{}\" | Select-Object Name,ObjectClass | Sort-Object -Property Name | ForEach-Object { ConvertTo-Json @($_) }", group_name);
    ```
    In this way, the parameter `group_name` will be read from the plugin section from the selected config file. The function `validate_windows_sam_account_name` has been properly crafted in the file `app/src/validator.rs` to verify the validity of the group name:

    ```rust
    pub fn validate_windows_sam_account_name(text: &str) -> bool {
        check_regex(r"^[^/\\\[\]:;|=,+?<>]+$", text)
    }
    ```

6. Replace `Ok(())` (Line 30) with the correct output processing function. Since the output of the command will be already a JSON object so the needed function is `self._convert_json_string(output)`.

Finally, the code should be this:

```rust
use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};
use crate::validator::validate_windows_sam_account_name;

pub struct WindowsDomainGroup {}

impl Plugin for WindowsDomainGroup {
    fn name(&self) -> &str {
        &"windows_domain_group"
    }

    fn description(&self) -> &str {
        &"Domain group members"
    }

    fn os(&self) -> OS {
        OS::Windows
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = format!("Get-ADGroupMember -Identity \"{}\" | Select-Object Name,ObjectClass | Sort-Object -Property Name | ForEach-Object {{ ConvertTo-Json @($_) }}", _config.get_string_setting(self.name(), "group_name"));
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._convert_json_string(output)
    }
}

impl WindowsDomainGroup {
    pub fn new() -> Self {
        WindowsDomainGroup {}
    }
}
```

### 3. Unlocking

The plugin logic has been defined but the plugin must be linked to the main application logic, so:

1. Add the line `pub mod domain_group;` to the `app/src/plugins/linux/mod.rs` file.

2. Update the `execute` method in the `app/src/lib.rs` file:

    - Below the comment `// Import Windows plugins` add the line `domain_group::WindowsDomainGroup,`
    - Below the comment `// Instantiate Windows plugins` add the line `let windows_domain_group = WindowsDomainGroup::new();`
    - Below the comment `// Execute Windows plugins` add the line `&windows_domain_group,`

Finally, the code should be like the following:

```rust
if #[cfg(target_os = "windows")] {
    use crate::plugins::windows::{
        // Import Windows plugins
        users::WindowsUsers,
        // ...
        domain_group::WindowsDomainGroup, // <- New module
        // ...
    };

    // Instantiate Windows plugins
    let windows_users = WindowsUsers::new();
    // ...
    let windows_domain_group = WindowsDomainGroup::new(); // <- New module
    // ...
    let plugins: Vec<&dyn Plugin> = vec![
                                            // Execute Windows plugins
                                            &windows_users,
                                            // ...
                                            &windows_domain_group, // <- New module
                                            // ...
                                        ];
    } else if #[cfg(target_os = "linux")] {
```


### 4. Configuration

In order to enable/disable and configure the plugin execution, it is necessary to add to the `config.ini` INI file the following block:

```ini
[windows_domain_group]
enabled    = true
group_name = "Domain Admins"
```

The default group to be queried has been set to "Domain Admins".


### 5. Re-building

Execute the following command to rebuild the framework (requires Docker) to include the new plugin:

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh build
```

The Docker image `peco602/rust-universal-compiler` will be downloaded (it can take some time) and used to rebuild the Rust code for all the platforms.