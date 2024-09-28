package main

import (
	natslistener "go-mono/internal/nats_listener"
	"go-mono/pkg/config_handler"
)

func main() {
	config, _err := config_handler.NewEnvVarHandler()
	if _err != nil {
		panic(_err)
	}
	config.LoadYaml("/Users/alex/git/monorepo/apps/go/configs/config.yaml")

	natsAddr, _err := config.Get("nats.addr")
	if _err != nil {
		panic(_err)
	}

	subject := "test"
	_err = natslistener.Listen(natsAddr, subject)
	if _err != nil {
		panic(_err)
	}
}
