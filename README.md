<div align="center">
  <img src=".github/cover.png" alt="mesa-dev" width="100%" />
</div>

<div align="center">
  <h1>mesa-dev</h1>
  <p><strong>Rust SDK for the <a href="https://mesa.dev">mesa.dev</a> API.</strong></p>

  <a href="https://crates.io/crates/mesa-dev"><img src="https://img.shields.io/crates/v/mesa-dev" alt="crates.io" /></a>
  <a href="https://docs.rs/mesa-dev"><img src="https://img.shields.io/docsrs/mesa-dev" alt="docs.rs" /></a>
  <a href="https://github.com/mesa-dot-dev/sdk-rust/blob/main/LICENSE"><img src="https://img.shields.io/crates/l/mesa-dev" alt="License" /></a>
  <a href="https://github.com/mesa-dot-dev/sdk-rust/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/mesa-dot-dev/sdk-rust/ci.yml?branch=main&label=CI" alt="CI" /></a>
  <a href="https://discord.gg/2vvEJFrCHV"><img src="https://img.shields.io/badge/Discord-Join%20us-5865F2?logo=discord&logoColor=white" alt="Discord" /></a>
</div>

---

## Install

```sh
cargo add mesa-dev
```

## Quick Start

```rust
use mesa_dev::apis::configuration::Configuration;
use mesa_dev::apis::repos_api;

let config = Configuration {
    bearer_access_token: Some("your-token".to_string()),
    ..Configuration::default()
};

let repos = repos_api::get_by_org_repos(&config, "my-org", None, None).await?;
```

## Documentation

Full API reference is available on [docs.rs](https://docs.rs/mesa-dev/latest/mesa_dev/).

## Authentication

The SDK supports three authentication methods via the `Configuration` struct:

**Bearer token** (most common):

```rust
let config = Configuration {
    bearer_access_token: Some("your-token".to_string()),
    ..Configuration::default()
};
```

**Basic auth**:

```rust
let config = Configuration {
    basic_auth: Some(("username".to_string(), Some("password".to_string()))),
    ..Configuration::default()
};
```

**API key**:

```rust
use mesa_dev::apis::configuration::{ApiKey, Configuration};

let config = Configuration {
    api_key: Some(ApiKey {
        prefix: Some("Bearer".to_string()),
        key: "your-api-key".to_string(),
    }),
    ..Configuration::default()
};
```

## License

[MIT](LICENSE)
