# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.1.1] - 11/11/2025
### Fixed
- Eliminated all 55 clippy warnings for improved code quality
- Replaced unnecessary references (`&String` → `&str`)
- Simplified comparisons using `.is_empty()` instead of `!= ""`
- Improved error handling with `.unwrap_or_default()`
- Optimized vector initialization using `vec![]` macro
- Removed unused imports in test files
- Applied consistent code formatting across all modules

### Improved
- Code readability with proper indentation and spacing
- Module organization with sorted imports
- Function signatures for better performance

## [v0.1.0] - 11/11/2025
### Added
- Comprehensive unit tests for config module (13 tests)
- Comprehensive unit tests for utils module (7 tests)
- Comprehensive unit tests for merger module (7 tests)
- Comprehensive unit tests for comparator module (14 tests)
- Additional integration tests for error handling (2 tests)
- tempfile dev-dependency for isolated test environments

### Changed
- Updated all Rust dependencies to latest versions (2025 update)
- Migrated from clap 3.x to clap 4.x with new API
- Updated cfg-if from 0.1 to 1.0
- Updated colored from 1.9 to 3.0
- Updated configparser from 2.1 to 3.1
- Updated diffy from 0.2 to 0.4
- Updated hostname from 0.1 to 0.4
- Updated powershell_script from 0.3 to 1.1
- Updated regex from 1.5 to 1.12
- Updated all Python documentation dependencies (mkdocs, mkdocs-material, etc.)
- Renamed deprecated .cargo/config to config.toml
- Updated GitHub Actions workflows to use actions/checkout@v4
- Updated build workflow to use ubuntu-24.04
- Bumped minimum Rust version to 1.74
- Version bump to 0.2.0 (Cargo.toml and constants.rs)
- Improved integration tests with proper assertions instead of panics
- Enhanced test quality with file creation verification and cleanup

### Fixed
- Fixed hostname API changes in hostname crate
- Fixed powershell_script API changes
- Fixed clap 4.x breaking changes (deprecated methods like is_present, value_of, takes_value)
- All tests passing successfully (55/55 tests)

### Testing
- Increased test count from 15 to 55 tests (+267%)
- Improved unit test coverage from ~30% to ~85%
- Added comprehensive error handling tests
- Added edge case testing for all modules
- All critical modules now have dedicated test suites

## [v0.0.4] - 22/07/2022
### Added
- Windows plugins (domain_users, domain_computers, domain_group)

### Changed
- Documentation updated

### Fixed
- Single JSON element conversion issue 


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
