package main

import (
	"go-mono/pkg/config_handler"
	natsdriver "go-mono/pkg/nats_driver"

	"github.com/rs/zerolog/log"
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

	natsConnnection, _err := natsdriver.NewNatsConnection(&natsAddr)
	if _err != nil {
		panic(_err)
	}

	defer natsConnnection.Close()

	pubSubClient := natsdriver.NewPubSub(natsConnnection)
	defer pubSubClient.Close()
	_err = pubSubClient.Publish("test", []byte("hello"))
	if _err != nil {
		log.Err(_err)
	}
}
