cargo build --target=wasm32-wasi
WASM_FILE="target/wasm32-wasi/debug/custom_framework_wasm.wasm"
cp $WASM_FILE ./bindings/go/lib
cp $WASM_FILE ./bindings/rust/lib
