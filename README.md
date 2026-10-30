# Design App

This is a 2D vector based design app for web and desktop (via web view).

The frontend is React, the backend is Rust (via WASM). It uses WebGPU through the `wgpu` package, and uses `vello` for 2D rendering.

## Development

1. Build the Rust code and compile the WASM + bindings. [Check README in renderer](crates\renderer\README.md) for commands.
1. Run the web app: `pnpm start:web`
