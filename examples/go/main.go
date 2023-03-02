package main

import (
	"github.com/proohit/custom-framework-go/pkg/module"
	server "github.com/proohit/custom-framework-go/pkg/server"
)

func main() {
	var server = server.New()
	server.AddRoute(module.RouteHandler{
		Path: "/",
		Handler: func(requestBody string) string {
			return "Hello World"
		},
	})
	server.Start()
}
