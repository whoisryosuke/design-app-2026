## Development

Build renderer and generate WASM + JS bindings:

```shell
cargo build -p renderer --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/renderer.wasm --target web --out-dir pkg
```
