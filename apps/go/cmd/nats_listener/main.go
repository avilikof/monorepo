package main

import (
	natslistener "go-mono/internal/nats_listener"
	"go-mono/pkg/config_handler"
)

func main() {
	config, err := config_handler.NewEnvVarHandler()
	if err != nil {
		panic(err)
	}
	config.LoadYaml("/Users/alex/git/monorepo/apps/go/configs/config.yaml")

	natsAddr, err := config.Get("nats.addr")
	if err != nil {
		panic(err)
	}

	subject := "test"
	err = natslistener.Listen(natsAddr, subject)
	if err != nil {
		panic(err)
	}
}
