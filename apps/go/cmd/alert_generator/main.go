package main

import (
	alertgenerator "go-mono/internal/alert_generator"
	"go-mono/pkg/config_handler"
	"log"
	"time"

	natsdriver "go-mono/pkg/nats_driver"
)

func main() {
	config, err := config_handler.NewEnvVarHandler()
	if err != nil {
		log.Fatalln(err)
	}
	config.LoadYaml("/Users/alex/git/monorepo/apps/go/configs/config.yaml")

	natsAddr, err := config.Get("nats.addr")
	if err != nil {
		log.Fatal(err)
	}
	natsConn, err := natsdriver.NewNatsConnection(&natsAddr)
	if err != nil {
		panic(err)
	}
	defer natsConn.Close()
	pubSub := natsdriver.NewPubSub(natsConn)
	for range 100 {
		err := alertgenerator.PublishAlert("test", pubSub)
		if err != nil {
			panic(err)
		}
		time.Sleep(time.Second)

	}
	time.Sleep(10 * time.Second)
}
