package server

import (
	"encoding/json"
	"os"
	"path"
	"path/filepath"
	"runtime"

	"github.com/second-state/WasmEdge-go/wasmedge"
)

var routes = map[int32]func(map[string]interface{}) string{
	0: func(requestBody map[string]interface{}) string {
		res, _ := json.Marshal(requestBody)
		return string(res)
	},
	1: func(requestBody map[string]interface{}) string {
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
	var requestHandlerIndex = params[0].(int32)
	var requestPointer = params[1].(int32)
	var requestLength = params[2].(int32)

	mem := callframe.GetModule().FindMemory("memory")
	var requestBodyString = getStringFromPointer(requestPointer, requestLength, mem)

	var requestBody map[string]interface{}
	json.Unmarshal([]byte(requestBodyString), &requestBody)

	var res = routes[requestHandlerIndex](requestBody)
	var resPtr = getStringPointer(res, _vm)

	return []interface{}{resPtr}, wasmedge.Result_Success
}

func New() {
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

	// register external request handler function
	var params = []wasmedge.ValType{
		wasmedge.ValType_I32,
		wasmedge.ValType_I32,
		wasmedge.ValType_I32,
	}
	var returns = []wasmedge.ValType{
		wasmedge.ValType_I32,
	}
	funcAddType := wasmedge.NewFunctionType(
		params,
		returns)
	host_handle_request_external := wasmedge.NewFunction(funcAddType, handle_request_external, nil, 0)
	var mod = wasmedge.NewModule("env")
	mod.AddFunction("handle_request_external", host_handle_request_external)
	vm.RegisterModule(mod)

	//load module
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
