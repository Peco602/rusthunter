# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.0.4] - TBD
### Added
- Windows plugins (domain_users, domain_computers, domain_group)

### Changed
- Documentation updated

### Fixed
- PowerShell ConvertTo-Json for single element issue 

## [v0.0.3] - 22/07/2022
### Added
- MITRE ATT&CK mapping
- Snapshot tagging
- Linux plugins (crontab, dns, suid, guid, promisc)

### Changed
- Documentation updated
- Validation testing logic

### Fixed
- macOS executable creation issue
- General bugfix


## [v0.0.2] - 03/07/2022
### Added
- Snapshot comparison based on statistics
- Snapshot comparison filtering by host and plugin
- macOS support
- Sample plugin for custom development

### Changed
- Snapshot comparison method
- Uploaded base builder and launcher docker images to DockerHub
- Documentation updated
- CI workflows updated

### Fixed
- Windows installation bug
- Host file encryption logic

### Security
- Hosts inventory file encryption


## [v0.0.1] - 15/05/2022
- First release
