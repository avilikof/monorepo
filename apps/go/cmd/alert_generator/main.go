package main

import (
	alertgenerator "go-mono/internal/alert_generator"
	"go-mono/pkg/config_handler"
	"log"
	"time"

	natsdriver "go-mono/pkg/nats_driver"
)

func main() {
	config, _err := config_handler.NewEnvVarHandler()
	if _err != nil {
		log.Fatalln(_err)
	}
	config.LoadYaml("/Users/alex/git/monorepo/apps/go/configs/config.yaml")

	natsAddr, _err := config.Get("nats.addr")
	if _err != nil {
		log.Fatal(_err)
	}
	natsConn, _err := natsdriver.NewNatsConnection(&natsAddr)
	if _err != nil {
		panic(_err)
	}
	defer natsConn.Close()
	pubSub := natsdriver.NewPubSub(natsConn)
	for range 100 {
		_err := alertgenerator.PublishAlert("test", pubSub)
		if _err != nil {
			panic(_err)
		}
		time.Sleep(time.Second)

	}
	time.Sleep(10 * time.Second)
}
