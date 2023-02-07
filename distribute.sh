cargo build --target=wasm32-wasi --release
WASM_FILE="target/wasm32-wasi/release/custom_framework_wasm.wasm"
cp $WASM_FILE ./bindings/go/lib
cp $WASM_FILE ./bindings/rust/lib
cp $WASM_FILE ./bindings/java/custom_framework/src/main/resources
