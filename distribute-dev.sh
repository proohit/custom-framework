cargo build --target=wasm32-wasi
WASM_FILE_NAME="custom_framework_wasm.wasm"
WASM_FILE="target/wasm32-wasi/debug/$WASM_FILE_NAME"
cp $WASM_FILE ./bindings/go/lib/$WASM_FILE_NAME
cp $WASM_FILE ./bindings/rust/lib/$WASM_FILE_NAME
cp $WASM_FILE ./bindings/java/custom_framework/src/main/resources/$WASM_FILE_NAME
