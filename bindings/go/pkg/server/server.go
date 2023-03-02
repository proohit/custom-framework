package server

import (
	"fmt"
	"strings"

	"github.com/proohit/custom-framework-go/pkg/module"
)

type Server struct {
	moduleWrapper module.ModuleWrapper
	routes        module.RouteHandlerMap
}

func New() Server {
	return Server{
		routes: module.RouteHandlerMap{},
	}
}

func (server *Server) Start() {
	server.moduleWrapper = module.New(server.routes)
	var stringifiedRoutes = []string{}
	for index, route := range server.routes {
		stringifiedRoutes = append(stringifiedRoutes, fmt.Sprintf("%d:%s", index, route.Path))
	}
	var stringifiedRoutesString = strings.Join(stringifiedRoutes, ",")

	server.moduleWrapper.Start(stringifiedRoutesString)
}

func (server *Server) AddRoute(handler module.RouteHandler) {
	server.routes[int32(len(server.routes))] = handler
}
