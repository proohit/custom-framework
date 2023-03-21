package module

import (
	"os"
	"path"
	"path/filepath"
	"runtime"

	"github.com/second-state/WasmEdge-go/wasmedge"
)

type ModuleWrapper struct {
	vm *wasmedge.VM
}

func New(requestHandlers RouteHandlerMap) ModuleWrapper {
	// Set not to print debug info
	wasmedge.SetLogErrorLevel()

	// Create configure
	var conf = wasmedge.NewConfigure(wasmedge.WASI)

	// Create VM with configure
	var vm = wasmedge.NewVMWithConfig(conf)
	var moduleWrapper = ModuleWrapper{vm}
	// Init WASI
	var wasi = vm.GetImportModule(wasmedge.WASI)
	wasi.InitWasi(
		os.Args[1:],     // The args
		os.Environ(),    // The envs
		[]string{".:."}, // The mapping preopens
	)

	var envModule = GetEnvModule(CreateHandleRequestExternalHostFunction(&moduleWrapper, requestHandlers))
	vm.RegisterModule(envModule)

	//load module
	_, currentFile, _, _ := runtime.Caller(0)
	currentDir := path.Dir(currentFile)
	wasmPath := filepath.Join(path.Dir(path.Dir(currentDir)), CustomFrameworkModulePath)
	vm.LoadWasmFile(wasmPath)
	vm.Validate()
	vm.Instantiate()

	return moduleWrapper
}

func (moduleWrapper *ModuleWrapper) GetStringPointer(data string) int32 {
	lengthOfData := len(data)

	allocateResult, _ := moduleWrapper.vm.Execute(FunctionAllocate, int32(lengthOfData+1))
	inputPointer := allocateResult[0].(int32)

	module := moduleWrapper.vm.GetActiveModule()
	mem := module.FindMemory(MemoryName)
	memData, _ := mem.GetData(uint(inputPointer), uint(lengthOfData+1))
	copy(memData, data)

	memData[lengthOfData] = 0

	return inputPointer
}

func (moduleWrapper *ModuleWrapper) GetStringFromPointer(ptr int32, len int32) string {
	mem := moduleWrapper.vm.GetActiveModule().FindMemory(MemoryName)
	memData, _ := mem.GetData(uint(ptr), uint(len))
	return string(memData[0:len])
}

func (moduleWrapper *ModuleWrapper) Start(routes string) {
	var mappingPtr = moduleWrapper.GetStringPointer(routes)
	moduleWrapper.vm.Execute(FunctionStart, mappingPtr)
}
