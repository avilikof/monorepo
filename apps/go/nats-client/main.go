package main

import (
	"fmt"

	"github.com/avilikof/monorepo/libs/go/alert_entity"
	"github.com/avilikof/monorepo/libs/go/nats_driver"
)

func main() {
	// Connect to a server

	url := "nats://192.168.32.161:4222"
	alert := alert_entity.RandomAlert(1000)
	alertByte, err := alert.ToByte()
	if err != nil {
		panic(err)
	}
	natsClient := nats_driver.DefaultClient(&url)
	topic := "test"

	natsClient.Push(&topic, alertByte)
}

func publishPayloadWithNats(payload []byte) {
	fmt.Println("hello")
}

func getRandomAlert(n int64) alert_entity.AlertEntity {
	return alert_entity.RandomAlert(n)
}
