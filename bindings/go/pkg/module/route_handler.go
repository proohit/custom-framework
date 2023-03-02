package module

type RouteHandler struct {
	Path    string
	Handler func(string) string
}

type RouteHandlerMap map[int32]RouteHandler
