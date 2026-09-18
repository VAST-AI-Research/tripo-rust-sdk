# Releasing

Publishing runs from `.github/workflows/publish.yml` using crates.io Trusted
Publishing. There is no `CARGO_REGISTRY_TOKEN` stored as a repository secret:
`rust-lang/crates-io-auth-action` exchanges the workflow's OIDC token for a
30-minute crates.io token and revokes it when the job finishes.

## Cutting a release

```sh
# bump `version` in Cargo.toml, then:
cargo test && cargo clippy --all-targets -- -D warnings
git commit -am "Release X.Y.Z"
git tag vX.Y.Z
git push origin main --tags
```

Pushing the `v*` tag triggers the workflow, which refuses to publish if the tag
does not match the `version` in `Cargo.toml`, then runs `cargo fmt --check`,
clippy, and the test suite before publishing.

The `User-Agent` derives from `CARGO_PKG_VERSION`, so it tracks `Cargo.toml`
automatically.

## One-time setup (already done)

On [crates.io](https://crates.io/crates/tripo3d-sdk/settings) → **Trusted
Publishing** → **Add a new publisher** → **GitHub**:

| Field | Value |
| --- | --- |
| Repository owner | `VAST-AI-Research` |
| Repository name | `tripo-rust-sdk` |
| Workflow filename | `publish.yml` |
| Environment | *(blank)* |

Trusted Publishing cannot bootstrap a brand-new crate — the first version must
be published manually, which for this crate already happened at `0.1.0`.
Renaming `publish.yml` breaks publishing until the entry is updated to match.
