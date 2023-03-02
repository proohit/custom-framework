package module

import "github.com/second-state/WasmEdge-go/wasmedge"

func GetEnvModule(handleRequestExternal HandleRequestExternal) *wasmedge.Module {
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
	host_handle_request_external := wasmedge.NewFunction(funcAddType, handleRequestExternal.Handle_request_external, nil, 0)
	var mod = wasmedge.NewModule("env")
	mod.AddFunction("handle_request_external", host_handle_request_external)
	return mod
}
