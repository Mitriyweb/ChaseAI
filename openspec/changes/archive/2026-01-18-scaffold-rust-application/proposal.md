# Change: Scaffold Rust Application Skeleton

## Why
This change initializes the core Rust application for ChaseAI. It provides the foundation for building the execution platform, following the mission to create a controlled environment for AI agents.

## What Changes
- [NEW] Initialize `Cargo.toml` with `app` library and custom source/test paths.
- [NEW] Create `src/rs/app.rs` as the main library entry point.
- [NEW] Create `src/rs/main.rs` as the executable entry point (includes `app`).
- [NEW] Create `src/test/rs/app.rs` for integration tests.

## Impact
- Affected specs: `app` (new capability)
- Affected code: `Cargo.toml`, `src/rs/`, `src/test/rs/`
