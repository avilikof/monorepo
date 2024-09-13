package main

import (
	"fmt"
	"go-mono/pkg/alerts/alert_entity"
	"go-mono/pkg/config_handler"
	natsdriver "go-mono/pkg/nats_driver"
	"log"

	"github.com/nats-io/nats.go"
)

const subject string = "test"

func main() {
	natsClient, err := natsdriver.NewNatsConnection(getNatsAddr())
	if err != nil {
		panic(err)
	}
	defer natsClient.Close()

	subClient := natsdriver.NewPubSub(natsClient)
	subClient.Subscribe(subject, encodeAlert)
}

func getNatsAddr() *string {
	config, err := config_handler.NewEnvVarHandler()
	if err != nil {
		log.Fatalln(err)
	}
	config.LoadYaml("/Users/alex/git/monorepo/apps/go/configs/config.yaml")

	natsAddr, err := config.Get("nats.addr")
	if err != nil {
		log.Fatal(err)
	}
	return &natsAddr
}

func encodeAlert(msg *nats.Msg) {
	alert, err := alert_entity.NewAlertEntityFromBytes(msg.Data)
	if err != nil {
		panic(err)
	}
	if alert.GetState() == "firing" {
		fmt.Println("Alert firing")
		return
	}
	fmt.Println("Alert resolved")
}
