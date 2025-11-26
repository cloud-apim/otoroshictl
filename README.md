# otoroshictl

a CLI to manage your [Otoroshi](https://github.com/MAIF/otoroshi) clusters with style ;) otoroshictl is a CLI that can interact with the admin api of an otoroshi cluster.

You can also use it to expose local process through the otoroshi remote tunnels feature and as an universal sidecar to create a service mesh based on otoroshi. otoroshictl also provide a nice integration with Cloud APIM.

otoroshictl is an open-source tool proudly provided by Cloud APIM (https://www.cloud-apim.com). Cloud APIM is a company that provides managed Otoroshi clusters and Wasmo instances perfectly configured and optimized, ready in seconds. The sources of otoroshictl are available on github at https://github.com/cloud-apim/otoroshictl

## Documentation

the full documentation for `otoroshictl` is available [here](https://cloud-apim.github.io/otoroshictl/docs/overview)

## Installation

```sh
cargo install otoroshictl
```

or download a pre-built binary from https://github.com/cloud-apim/otoroshictl/releases

## Testing

The project includes a comprehensive test suite with unit tests and integration tests against a real Otoroshi instance.

### Quick Start

[!WARNING] Tests will overwrite your local configurations if any. Don't run tests on a production setup.

```sh
# Run unit tests (no Docker required)
make test

# Run all tests with Otoroshi (Docker required)
make test-all
```

### Available Commands

| Command | Description |
|---------|-------------|
| `make test` | Run unit tests only (fast) |
| `make test-unit` | Same as `make test` |
| `make check` | Run linting, formatting checks and unit tests |
| `make otoroshi-start` | Start Otoroshi in Docker |
| `make otoroshi-stop` | Stop Otoroshi |
| `make test-integration` | Run integration tests (Otoroshi must be running) |
| `make test-all` | Full workflow: build, start Otoroshi, run all tests, stop |

## Demo

<p align="center">
  <img width="750" src="./demo.svg">
</p>
