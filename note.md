compile rust:

rustup target add wasm32-wasi

rustc hello.rs --target wasm32-wasi




Compile Go:

GOARCH=wasm GOOS=js go build -o form.wasm form.go




run:

wasmtime hello.wasm

|¡Hola, mundo!
