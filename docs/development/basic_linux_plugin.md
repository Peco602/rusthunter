# How to develop a basic Linux plugin

In this tutorial it will be shown how to develop a basic plugin that collects the list of `root` users on a Linux machine. It can be considered as a guideline to create additional plugins for Linux.

### 1. Scaffolding

The first steps consists of creating the structure of the plugin based on the sample plugin already made available. The plugin name will be `linux_root` so the command to be executed is the following:

```console
user@master-node:~/rusthunter$ cp -r app/src/plugins/sample app/src/plugins/linux/root
```

The plugin folder contains the `mod.rs` file which provides the plugin logic:

```console
user@master-node:~/rusthunter$ ls -al app/src/plugins/linux/root
mod.rs
```

### 2. Customization

The file `app/src/plugins/linux/root/mod.rs` must be properly customized to collect the desired data on the target machine.

1. Replace all the occurrencies of the struct name `SamplePlugin` with `LinuxRoot`. It can be easily done via the command:

```console
user@master-node:~/rusthunter$ sed -i 's/SamplePlugin/LinuxRoot/g' app/src/plugins/linux/root/mod.rs
```

2. Replace the plugin name `sample_plugin` with `linux_root` (Line 10).

3. Replace the plugin description `Sample description` with `Local root users` (Line 14).

4. Replace the plugin operating system `OS::Unknown` with `OS::Linux` (Line 18).

5. Replace the plugin command `sample command` with `cat /etc/passwd | grep :0: |  cut -d : -f 1 | sort` (Line 22).

6. Replace `Ok(())` (Line 30) with the correct output processing function. Since the output of the command will be a simple list the needed function is `self._split_list(output)`.

Finally, the code should be this:

```rust
use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxRoot {}

impl Plugin for LinuxRoot {
    fn name(&self) -> &str {
        &"linux_root" 
    }

    fn description(&self) -> &str {
        &"Local root users"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = "cat /etc/passwd | grep :0: | cut -d : -f 1 | sort";
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl LinuxRoot {
    pub fn new() -> Self {
        LinuxRoot {}
    }
}
```

### 3. Unlocking

The plugin logic has been defined but the plugin must be linked to the main application logic, so:

1. Add the line `pub mod root;` to the `app/src/plugins/linux/mod.rs` file.

2. Update the `execute` method in the `app/src/lib.rs` file:

    - Below the comment `// Import Linux plugins` add the line `root::LinuxRoot,`
    - Below the comment `// Instantiate Linux plugins` add the line `let linux_root = LinuxRoot::new();`
    - Below the comment `// Execute Linux plugins` add the line `&linux_root,`

Finally, the code should be like the following:

```rust
} else if #[cfg(target_os = "linux")] {
use crate::plugins::linux::{
    // Import Linux plugins
    users::LinuxUsers,
    // ...
    root::LinuxRoot, // <- New module
    // ...
    tcp_listen::LinuxTCPListen,
};

// Instantiate Linux plugins
let linux_users = LinuxUsers::new();
// ...
let linux_root = LinuxRoot::new(); // <- New module
// ...
let linux_tcp_listen = LinuxTCPListen::new();
let plugins: Vec<&dyn Plugin> = vec![
                                        // Execute Linux plugins
                                        &linux_users,
                                        // ...
                                        &linux_root, // <- New module
                                        &linux_tcp_listen,
                                        // ...
                                    ];
} else if #[cfg(target_os = "macos")] {
```

### 4. Configuration

In order to enable/disable the plugin execution, it is necessary to add to the `config.ini` INI file the following block:

```ini
[linux_root]
enabled = true
```

### 5. Re-building

Execute the following command to rebuild the framework (requires Docker) to include the new plugin:

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh build
```

The Docker image `peco602/rust-universal-compiler` will be downloaded (it can take some time) and used to rebuild the Rust code for all the platforms.