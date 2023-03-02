package module

import (
	"github.com/second-state/WasmEdge-go/wasmedge"
)

type HandleRequestExternal struct {
	moduleWrapper           *ModuleWrapper
	routes                  RouteHandlerMap
	Handle_request_external func(data interface{}, callframe *wasmedge.CallingFrame, params []interface{}) ([]interface{}, wasmedge.Result)
}

func CreateHandleRequestExternalHostFunction(moduleWrapper *ModuleWrapper, routes RouteHandlerMap) HandleRequestExternal {
	return HandleRequestExternal{
		moduleWrapper: moduleWrapper,
		routes:        routes,
		Handle_request_external: func(data interface{}, callframe *wasmedge.CallingFrame, params []interface{}) ([]interface{}, wasmedge.Result) {
			var requestHandlerIndex = params[0].(int32)
			var requestPointer = params[1].(int32)
			var requestLength = params[2].(int32)

			var requestBodyString = moduleWrapper.GetStringFromPointer(requestPointer, requestLength)

			var res = routes[requestHandlerIndex].Handler(requestBodyString)
			var resPtr = moduleWrapper.GetStringPointer(res)

			return []interface{}{resPtr}, wasmedge.Result_Success
		},
	}
}
