package server

import (
	"encoding/json"
	"fmt"
	"os"
	"path"
	"path/filepath"
	"runtime"

	"github.com/second-state/WasmEdge-go/wasmedge"
)

type response struct {
	test string
}

var routes = map[int32]func() string{
	0: func() string {
		return "Hello World"
	},
	1: func() string {
		return "Test"
	},
}

var _vm *wasmedge.VM = nil

func getStringPointer(data string, vm *wasmedge.VM) int32 {
	lengthOfData := len(data)
	// Allocate memory for the subject, and get a pointer to it.
	// Include a byte for the NULL terminator we add below.
	allocateResult, _ := vm.Execute("allocate", int32(lengthOfData+1))
	inputPointer := allocateResult[0].(int32)

	// Write the subject into the memory.
	mod := vm.GetActiveModule()
	mem := mod.FindMemory("memory")
	memData, _ := mem.GetData(uint(inputPointer), uint(lengthOfData+1))
	copy(memData, data)

	// C-string terminates by NULL.
	memData[lengthOfData] = 0

	return inputPointer
}

func getStringFromPointer(ptr int32, len int32, mem *wasmedge.Memory) string {
	memData, _ := mem.GetData(uint(ptr), uint(len))
	return string(memData[0:len])
}

func handle_request_external(data interface{}, callframe *wasmedge.CallingFrame, params []interface{}) ([]interface{}, wasmedge.Result) {
	var pathIdx = params[0].(int32)
	var req_ptr = params[1].(int32)
	var req_len = params[2].(int32)
	mem := callframe.GetModule().FindMemory("memory")
	var req_body_string = getStringFromPointer(req_ptr, req_len, mem)
	fmt.Println(req_body_string)
	var req_body map[string]interface{}
	json.Unmarshal([]byte(req_body_string), &req_body)
	fmt.Println(json.Marshal(req_body))
	var res = routes[pathIdx]()
	var resPtr = getStringPointer(res, _vm)

	return []interface{}{resPtr}, wasmedge.Result_Success
}

func New() {
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

	var params = []wasmedge.ValType{
		wasmedge.ValType_I32,
		wasmedge.ValType_I32,
		wasmedge.ValType_I32,
	}
	var returns = []wasmedge.ValType{
		wasmedge.ValType_I32,
	}

	// Build Host Function's parameter and return value type
	funcAddType := wasmedge.NewFunctionType(
		params,
		returns)

	host_handle_request_external := wasmedge.NewFunction(funcAddType, handle_request_external, nil, 0)
	var mod = wasmedge.NewModule("env")
	mod.AddFunction("handle_request_external", host_handle_request_external)
	vm.RegisterModule(mod)
	_, currentFile, _, _ := runtime.Caller(0)
	currentDir := path.Dir(currentFile)
	wasmPath := filepath.Join(currentDir, "/lib/custom_framework_wasm.wasm")
	vm.LoadWasmFile(wasmPath)
	vm.Validate()
	vm.Instantiate()

	_vm = vm
}

func Start() {
	var mappingPtr = getStringPointer("0:/,1:/test", _vm)
	_vm.Execute("start", mappingPtr)
}
