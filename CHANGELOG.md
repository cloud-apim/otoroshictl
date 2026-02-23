# Changelog

All notable changes to `otoroshictl` are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [0.0.17] - 2026-02-22

### Added
- TCP tunnel support
- UDP tunnel support
- Support for Otoroshi consumer infos passthrough
- Test runner, test server and test route for integration testing

### Changed
- `remote_host` and `remote_port` are now optional
- Allow using different algorithms and keys for input and output in challenge proxy
- Change secret behavior when using public keys
- Headers option cleanup

---

## [0.0.16] - 2026-02-12

### Added
- Asymmetric algorithm support for the challenge proxy
- Signature algorithm option for the challenge proxy
- More environment variables support for the challenge proxy
- Gateway-specific rights to RBAC

---

## [0.0.15] - 2026-01-28

### Added
- `OTOROSHI_CHALLENGE_SECRET` environment variable support for the challenge command

### Changed
- Dependencies update (Q1 2026)

---

## [0.0.14] - 2026-01-15

### Fixed
- Version number displayed by `--version` flag

---

## [0.0.13] - 2025-12-03

### Added
- `toolbox open` command to open the Otoroshi UI in the browser

---

## [0.0.12] - 2025-12-03

### Added
- `toolbox add-mailer` command
- `exp` and `iss` claims verification in the challenge proxy

### Fixed
- Challenge proxy: use Otoroshi's default leeway value

---

## [0.0.11] - 2025-11-27

### Added
- `challenge` command for Otoroshi challenge/response proxy flows
- `toolbox mtls` command
- Command aliases
- Unit tests and CI pipeline for tests

### Changed
- Dependencies update

### Fixed
- Clippy warnings

---

## [0.0.10] - 2025-11-06

### Changed
- `api_version` field normalized to snake_case

---

## [0.0.9] - 2025-11-03

### Fixed
- Inbound proxy start

### Changed
- Fallback script added
- Documentation and version updated automatically

---

## [0.0.8] - 2025-11-01

### Added
- Config import from stdin (`otoroshictl import -`)

---

## [0.0.7] - 2025-11-01

### Changed
- Updated to a more recent async runtime

---

## [0.0.6] - 2025-11-01

### Added
- Clever Cloud configuration import (`toolbox clever-cloud-import`)

---

## [0.0.5] - 2024-06-13

### Added
- `--username` option for RBAC commands

---

## [0.0.4] - 2024-06-13

### Fixed
- RBAC command execution

---

## [0.0.3] - 2024-06-13

### Added
- Namespace handling

---

## [0.0.2] - 2024-04-29

### Added
- Documentation website
- Inline form notation support for resource definitions

---

## [0.0.1] - 2024-02-28

### Added
- Initial release
- CLI skeleton with Clap
- GitHub Actions release workflow

---

[0.0.17]: https://github.com/cloud-apim/otoroshictl/compare/0.0.16...0.0.17
[0.0.16]: https://github.com/cloud-apim/otoroshictl/compare/0.0.15...0.0.16
[0.0.15]: https://github.com/cloud-apim/otoroshictl/compare/0.0.14...0.0.15
[0.0.14]: https://github.com/cloud-apim/otoroshictl/compare/0.0.13...0.0.14
[0.0.13]: https://github.com/cloud-apim/otoroshictl/compare/0.0.12...0.0.13
[0.0.12]: https://github.com/cloud-apim/otoroshictl/compare/0.0.11...0.0.12
[0.0.11]: https://github.com/cloud-apim/otoroshictl/compare/0.0.10...0.0.11
[0.0.10]: https://github.com/cloud-apim/otoroshictl/compare/0.0.9...0.0.10
[0.0.9]: https://github.com/cloud-apim/otoroshictl/compare/0.0.8...0.0.9
[0.0.8]: https://github.com/cloud-apim/otoroshictl/compare/0.0.7...0.0.8
[0.0.7]: https://github.com/cloud-apim/otoroshictl/compare/0.0.6...0.0.7
[0.0.6]: https://github.com/cloud-apim/otoroshictl/compare/0.0.5...0.0.6
[0.0.5]: https://github.com/cloud-apim/otoroshictl/compare/0.0.4...0.0.5
[0.0.4]: https://github.com/cloud-apim/otoroshictl/compare/0.0.3...0.0.4
[0.0.3]: https://github.com/cloud-apim/otoroshictl/compare/0.0.2...0.0.3
[0.0.2]: https://github.com/cloud-apim/otoroshictl/compare/0.0.1...0.0.2
[0.0.1]: https://github.com/cloud-apim/otoroshictl/releases/tag/0.0.1
