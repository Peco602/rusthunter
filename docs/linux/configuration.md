# Linux :: Configuration

The `config.ini` configuration files can be modified to:

- Enable/disable plugins
- Modify plugin parameters

in relation to the data to be collected.

!!! note
    It is suggested to prepare in advance multiple configuration files and then to select each time the most suitable for the snapshot.
    
```ini
############## Linux ##############

[linux_crontab]
enabled = true

[linux_dns]
enabled = true

// ... skipped ...

############## Windows ##############

[windows_users]
enabled = true

[windows_administrators]
enabled = true

[windows_autoruns]
enabled             = true
boot_execute        = true
appinit_dlls        = false
explorer_addons     = false
sidebar_gadgets     = false
image_hijacks       = false
ie_addons           = false
known_dlls          = false
logon_startups      = true
wmi_entries         = true
winsock_protocol    = false
codecs              = false
printer_dlls        = false
lsa_providers       = false
autostart_services  = true
scheduled_tasks     = true
winlogon_entries    = true

// ... skipped ...

############## macOS ##############

[macos_users]
enabled   = true

// ... skipped ...

```
