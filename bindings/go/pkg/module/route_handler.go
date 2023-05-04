package module

type RouteHandler struct {
	Path    string
	Handler func(string) []byte
}

type RouteHandlerMap map[int32]RouteHandler
