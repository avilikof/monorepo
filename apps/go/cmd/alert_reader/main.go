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
	natsClient, _err := natsdriver.NewNatsConnection(getNatsAddr())
	if _err != nil {
		panic(_err)
	}
	defer natsClient.Close()

	subClient := natsdriver.NewPubSub(natsClient)
	subClient.Subscribe(subject, encodeAlert)
}

func getNatsAddr() *string {
	config, _err := config_handler.NewEnvVarHandler()
	if _err != nil {
		log.Fatalln(_err)
	}
	config.LoadYaml("/Users/alex/git/monorepo/apps/go/configs/config.yaml")

	natsAddr, _err := config.Get("nats.addr")
	if _err != nil {
		log.Fatal(_err)
	}
	return &natsAddr
}

func encodeAlert(msg *nats.Msg) {
	alert, _err := alert_entity.NewAlertEntityFromBytes(msg.Data)
	if _err != nil {
		panic(_err)
	}
	if alert.GetState() == "firing" {
		fmt.Println("Alert firing")
		return
	}
	fmt.Println("Alert resolved")
}
