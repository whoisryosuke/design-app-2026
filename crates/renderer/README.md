## Development

Build renderer and generate WASM + JS bindings:

```shell
pnpm build:wasm
```

Which essentially runs:

```shell
cargo build -p renderer --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/renderer.wasm --target web --out-dir crates/renderer/pkg
```

> Prefer running the `pnpm build:wasm` script in the root of project. That runs the commands above - as well as other useful things like generating TS types from Rust.

## Testing

### Rust

```shell
cargo test
```

Currently this only generates TS types from Rust types. But if you add any unit or integration tests, they should appear here.

### JavaScript

We test the JS bindings of the WASM using `vitest` and `playwright`.

```shell
pnpm test
```

This should boot up a Playwright browser.

The tests located in `/tests/` folder will automatically run. It also hot reloads, so feel free to edit tests and save to see changes immediately.

If it's your first time using Playwright, make sure to run `pnpm exec playwright install` to install the Chromium backend.

> Make sure to run the build process (`pnpm build:wasm` in root) every time you make changes to the Rust code to see changes in testing environments. Tests import directly from the build folder (`/pkg/`).
