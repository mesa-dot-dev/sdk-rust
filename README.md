# mesa-dev

Rust SDK for the [mesa.dev](https://mesa.dev) API — manage repositories, branches, commits, content, diffs, and API keys programmatically.

[API Documentation](https://docs.rs/mesa-dev)

## Installation

```sh
cargo add mesa-dev
```

## Quick Start

```rust,no_run
use mesa_dev::{Mesa, MesaError, models::CreateRepoRequest};

#[tokio::main]
async fn main() -> Result<(), MesaError> {
    let client = Mesa::new("my-api-key");

    // Create a repository
    let repo = client
        .repos("my-org")
        .create(&CreateRepoRequest {
            name: "my-repo".to_owned(),
            default_branch: None,
        })
        .await?;
    println!("Created repo: {}", repo.name);

    // List all branches (automatically paginates)
    let branches = client
        .branches("my-org", "my-repo")
        .list_all()
        .collect()
        .await?;
    println!("Found {} branches", branches.len());

    Ok(())
}
```

## Resources

| Resource | Accessor | Operations |
|----------|----------|------------|
| Repos | `client.repos(org)` | create, list, get, rename, delete |
| Branches | `client.branches(org, repo)` | create, list, delete |
| Commits | `client.commits(org, repo)` | create, list, get |
| Content | `client.content(org, repo)` | get (files and directories) |
| Diffs | `client.diffs(org, repo)` | get |
| Admin | `client.admin(org)` | API key management |

Paginated endpoints expose a `list_all()` method that returns a `PageStream` with lazy cursor-based iteration.

## HTTP Backends

The SDK is generic over its HTTP transport via the `HttpClient` trait.

| Feature | Backend | Async | Default |
|---------|---------|-------|---------|
| `reqwest-client` | [reqwest](https://docs.rs/reqwest) | yes | yes |
| `ureq-client` | [ureq](https://docs.rs/ureq) | no | no |

You can also bring your own backend by implementing `HttpClient`. See the [trait documentation](https://docs.rs/mesa-dev/latest/mesa_dev/trait.HttpClient.html) for a full guide.

## Retry

All requests are retried with exponential backoff and jitter (up to 3 attempts by default). Retryable conditions: HTTP 429, 5xx responses, timeouts, and connection errors. Retry parameters are configurable via `ClientBuilder`.

## Minimum Supported Rust Version

Rust **1.85+** (edition 2024).

---

> **Note:** A large part of this documentation was generated with the assistance of an LLM. If you spot inaccuracies, please open an issue.
