# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.2] - 2021-06-01

### Changed

- Block spacing in application for better compatibility with smaller screens

## [0.2.1] - 2021-05-31

### Added

- Color to response field

## [0.2.0] - 2021-05-30

### Changed

- Migration to tui-rs for more interactive usage

### Removed

- `ctrlc` dependency in favor of direct Ctrl + C detection and handling

## [0.1.1] - 2021-05-29

### Added

- More verbose messages

### Changed

- Fix [#1] by adding `::std::process::exit(0)` to `ctrlc` handler

## [0.1.0] - 2021-05-28

### Added
- Inital Release
