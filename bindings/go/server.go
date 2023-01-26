package server

import (
	"os"
	"path"
	"path/filepath"
	"runtime"

	"github.com/second-state/WasmEdge-go/wasmedge"
)

type Server struct {
	vm wasmedge.VM
}

func New() Server {
	// Expected Args[0]: program name (./bindgen_wasi)
	// Expected Args[1]: wasm or wasm-so file (rust_bindgen_wasi_lib_bg.wasm))

	// Set not to print debug info
	wasmedge.SetLogErrorLevel()

	// Create configure
	var conf = wasmedge.NewConfigure(wasmedge.WASI)

	// Create VM with configure
	var vm = wasmedge.NewVMWithConfig(conf)

	// Init WASI
	var wasi = vm.GetImportModule(wasmedge.WASI)
	wasi.InitWasi(
		os.Args[1:],     // The args
		os.Environ(),    // The envs
		[]string{".:."}, // The mapping preopens
	)
	_, currentFile, _, _ := runtime.Caller(0)
	currentDir := path.Dir(currentFile)
	wasmPath := filepath.Join(currentDir, "/lib/custom_framework_wasm.wasm")
	vm.LoadWasmFile(wasmPath)
	vm.Validate()
	vm.Instantiate()

	return Server{vm: *vm}
}

func (s Server) Start() {
	s.vm.Execute("start")
}
