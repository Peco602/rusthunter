# How to develop a Linux plugin with an external script

In this tutorial it will be shown how to develop a basic plugin that collects the list of all the `crontab` jobs on a Linux machine. Since it has not been found a one-fits-all one-liner to collect the data for all users, an external script will be employed. 

### 1. External script

The following script will be saved in a file called `crontab.sh` placed in the path `launcher/ansible/roles/linus/files`:

```bash
#!/bin/bash

#
# Source: https://stackoverflow.com/questions/134906/how-do-i-list-all-cron-jobs-for-all-users
#

# System-wide crontab file and cron job directory. Change these for your system.
CRONTAB='/etc/crontab'
CRONDIR='/etc/cron.d'

# Single tab character. Annoyingly necessary.
tab=$(echo -en "\t")

# Given a stream of crontab lines, exclude non-cron job lines, replace
# whitespace characters with a single space, and remove any spaces from the
# beginning of each line.
function clean_cron_lines() {
    while read line ; do
        echo "${line}" |
            egrep --invert-match '^($|\s*#|\s*[[:alnum:]_]+=)' |
            sed --regexp-extended "s/\s+/ /g" |
            sed --regexp-extended "s/^ //"
    done;
}

# Given a stream of cleaned crontab lines, echo any that don't include the
# run-parts command, and for those that do, show each job file in the run-parts
# directory as if it were scheduled explicitly.
function lookup_run_parts() {
    while read line ; do
        match=$(echo "${line}" | egrep -o 'run-parts (-{1,2}\S+ )*\S+')

        if [[ -z "${match}" ]] ; then
            echo "${line}"
        else
            cron_fields=$(echo "${line}" | cut -f1-6 -d' ')
            cron_job_dir=$(echo  "${match}" | awk '{print $NF}')

            if [[ -d "${cron_job_dir}" ]] ; then
                for cron_job_file in "${cron_job_dir}"/* ; do  # */ <not a comment>
                    [[ -f "${cron_job_file}" ]] && echo "${cron_fields} ${cron_job_file}"
                done
            fi
        fi
    done;
}

# Temporary file for crontab lines.
temp=$(mktemp) || exit 1

# Add all of the jobs from the system-wide crontab file.
cat "${CRONTAB}" | clean_cron_lines | lookup_run_parts >"${temp}" 

# Add all of the jobs from the system-wide cron directory.
cat "${CRONDIR}"/* | clean_cron_lines >>"${temp}"  # */ <not a comment>

# Add each user's crontab (if it exists). Insert the user's name between the
# five time fields and the command.
while read user ; do
    crontab -l -u "${user}" 2>/dev/null |
        clean_cron_lines |
        sed --regexp-extended "s/^((\S+ +){5})(.+)$/\1${user} \3/" >>"${temp}"
done < <(cut --fields=1 --delimiter=: /etc/passwd)

# Output the collected crontab lines. Replace the single spaces between the
# fields with tab characters, sort the lines by hour and minute, insert the
# header line, and format the results as a table.
cat "${temp}" |
    sed --regexp-extended "s/^(\S+) +(\S+) +(\S+) +(\S+) +(\S+) +(\S+) +(.*)$/\1\t\2\t\3\t\4\t\5\t\6\t\7/" |
    sort --numeric-sort --field-separator="${tab}" --key=2,1 |
    column -s"${tab}" -t

rm --force "${temp}"
```

### 2. Scaffolding

The plugin name will be `linux_crontab` so the command to be executed to create the empty plugin structure is the following:

```console
user@master-node:~/rusthunter$ cp -r app/src/plugins/sample app/src/plugins/linux/crontab
```

The plugin folder contains the `mod.rs` file which provides the plugin logic:

```console
user@master-node:~/rusthunter$ ls -al app/src/plugins/linux/crontab
mod.rs
```

### 3. Customization

The file `app/src/plugins/linux/crontab/mod.rs` must be properly customized to collect the desired data on the target machine.

1. Replace all the occurrencies of the struct name `SamplePlugin` with `LinuxCrontab`. It can be easily done via the command:

```console
user@master-node:~/rusthunter$ sed -i 's/SamplePlugin/LinuxCrontab/g' app/src/plugins/linux/crontab/mod.rs
```

2. Replace the plugin name `sample_plugin` with `linux_crontab` (Line 10).

3. Replace the plugin description `Sample description` with `Crontab jobs` (Line 14).

4. Replace the plugin operating system `OS::Unknown` with `OS::Linux` (Line 18).

5. Replace the plugin line `let command = "sample command";` with `let command = format!("{}/{}", _binary_directory, "crontab.sh");` (Line 22).

6. Replace `Ok(())` (Line 30) with the correct output processing function. Since the output of the command will be a simple list the needed function is `self._split_list(output)`.

Finally, the code should be this:

```rust
use serde_json::Value;

use crate::config::Config;
use crate::plugins::{Plugin, OS};

pub struct LinuxCrontab {}

impl Plugin for LinuxCrontab {
    fn name(&self) -> &str {
        &"linux_crontab" 
    }

    fn description(&self) -> &str {
        &"Crontab jobs"
    }

    fn os(&self) -> OS {
        OS::Linux
    }

    fn run(&self, _config: &Config, _binary_directory: &str) -> Result<Value, String> {
        let command = format!("{}/{}", _binary_directory, "crontab.sh");
        match self.execute_command(&command) {
            Ok(output) => self.process(&output),
            Err(e) => Err(e),
        }
    }

    fn process(&self, output: &str) -> Result<Value, String> {
        self._split_list(output)
    }
}

impl LinuxCrontab {
    pub fn new() -> Self {
        LinuxCrontab {}
    }
}
```

### 3. Unlocking

The plugin logic has been defined but the plugin must be linked to the main application logic, so:

1. Add the line `pub mod crontab;` to the `app/src/plugins/linux/mod.rs` file.

2. Update the `execute` method in the `app/src/lib.rs` file:

    - Below the comment `// Import Linux plugins` add the line `root::LinuxCrontab,`
    - Below the comment `// Instantiate Linux plugins` add the line `let linux_crontab = LinuxCrontab::new();`
    - Below the comment `// Execute Linux plugins` add the line `&linux_crontab,`

Finally, the code should be like the following:

```rust
} else if #[cfg(target_os = "linux")] {
use crate::plugins::linux::{
    // Import Linux plugins
    users::LinuxUsers,
    // ...
    root::LinuxCrontab, // <- New module
    // ...
    tcp_listen::LinuxTCPListen,
};

// Instantiate Linux plugins
let linux_users = LinuxUsers::new();
// ...
let linux_crontab = LinuxCrontab::new(); // <- New module
// ...
let linux_tcp_listen = LinuxTCPListen::new();
let plugins: Vec<&dyn Plugin> = vec![
                                        // Execute Linux plugins
                                        &linux_users,
                                        // ...
                                        &linux_crontab, // <- New module
                                        &linux_tcp_listen,
                                        // ...
                                    ];
} else if #[cfg(target_os = "macos")] {
```

### 4. Configuration

In order to enable/disable the plugin execution, it is necessary to add to the `config.ini` INI file the following block:

```ini
[linux_crontab]
enabled = true
```

### 5. Re-building

Execute the following command to rebuild the framework (requires Docker) to include the new plugin:

```console
user@master-node:~/rusthunter$ sudo ./rusthunter.sh build
```

The Docker image `peco602/rust-universal-compiler` will be downloaded (it can take some time) and used to rebuild the Rust code for all the platforms.