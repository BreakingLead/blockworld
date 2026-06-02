# AGENTS.md

## Architecture

Rust Cargo workspace (resolver = "2") with 3 crates, dependency order:

```
blockworld-utils  (types: Identifier, Registry, AM<T>, RR<T>)
  → blockworld-server  (Block, SubChunk, WorldAccess trait, packets)
    → blockworld-client  (binary, wgpu + winit + egui renderer)
```

The comprehensive architecture document is at `ARCHITECTURE.md`.

## Unusual directory layout

`blockworld-client/` has `main.rs` at the crate root directly — NOT under `src/`. Source modules live in `game/` and `renderer/` alongside it (no `src/` prefix). The `[[bin]]` section in `blockworld-client/Cargo.toml` sets `path = "main.rs"`.

## Commands

```sh
# Build everything
cargo build --workspace

# Run the client
cargo run -p blockworld-client

# Test everything (only 1 test exists, in atlas_image.rs)
cargo test --workspace

# Test a single crate
cargo test -p blockworld-client

# Lint (no rustfmt.toml or clippy.toml — use defaults)
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
```

## CI

GitHub Actions (`.github/workflows/rust.yml`) builds and tests everything from workspace root with `--workspace`. Includes `cargo fmt --check` and `cargo clippy` lint steps. If you add a new crate, update `Cargo.toml` members — the CI will pick it up automatically.

## Key facts

- Minecraft 1.16 reimplementation in Rust, targeting content parity with 1.12.2
- Uses `once_cell::sync::Lazy` for global registries (BLOCK_REGISTRY, BLOCK_ATLAS)
- `Identifier` stores `namespace:path` as a single `String`
- Chunk storage uses YZX layout: `index = y * 16 * 16 + z * 16 + x`
- Client binary has no `src/` dir — just `main.rs`, `game/`, `renderer/`
- `ARCHITECTURE.md` mentions a `blockworld-renderer` crate that does **not** exist in the workspace — it's a stale reference, ignore it
- The client must run from `blockworld-client/` working directory (set in `.vscode/launch.json`) to find `assets/`
- There is no `rust-toolchain` or `rustfmt.toml` — use whatever stable Rust is installed
