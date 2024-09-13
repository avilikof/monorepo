package main

import (
	"go-mono/pkg/config_handler"
	natsdriver "go-mono/pkg/nats_driver"

	"github.com/rs/zerolog/log"
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

	natsConnnection, err := natsdriver.NewNatsConnection(&natsAddr)
	if err != nil {
		panic(err)
	}

	defer natsConnnection.Close()

	pubSubClient := natsdriver.NewPubSub(natsConnnection)
	defer pubSubClient.Close()
	err = pubSubClient.Publish("test", []byte("hello"))
	if err != nil {
		log.Err(err)
	}
}
