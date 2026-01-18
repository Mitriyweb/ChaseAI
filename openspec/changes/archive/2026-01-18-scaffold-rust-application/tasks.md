## 1. Project Initialization
- [x] 1.1 Create `Cargo.toml` with `src/rs/app.rs` as the library path.
  - Set `[lib] path = "src/rs/app.rs"`
- [x] 1.2 Create `src/rs/app.rs` with a basic function.
- [x] 1.3 Create `src/test/rs/app.rs` and configure it as a test file in `Cargo.toml`.
- [x] 1.4 Add `anyhow` as a dependency.
- [x] 1.5 Update `package.json` to ensure `npm test` runs `cargo test`.

## 2. Verification
- [x] 2.1 Run `cargo build` to verify the structure.
- [x] 2.2 Run `cargo test` to verify the test location.
- [x] 2.3 Add executable entry point `src/rs/main.rs`.
- [x] 2.4 Verify `npm start` executes the application.
