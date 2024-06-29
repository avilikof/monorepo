package main

import (
	"fmt"
	"sync"

	"github.com/avilikof/monorepo/libs/go/alert_entity"
	"github.com/avilikof/monorepo/libs/go/nats_driver"
)

func main() {
	// Connect to a server

	url := "nats://192.168.32.161:4222"
	alert := alert_entity.RandomAlert(1000)
	// fmt.Println(alert)
	alertByte, err := alert.ToByte()
	if err != nil {
		panic(err)
	}
	natsClient := nats_driver.DefaultClient(&url)
	defer natsClient.Close()
	subject := "test"

	fmt.Println(natsClient.Info())
	fmt.Println(natsClient.Version())
	var wg sync.WaitGroup
	wg.Add(1)
	go reader(natsClient, &subject, &wg)
	err = natsClient.Push(&subject, alertByte)
	if err != nil {
		panic(err)
	}
	natsClient.Push(&subject, alertByte)
	natsClient.Push(&subject, alertByte)
	wg.Wait()
}

func getRandomAlert(n int64) alert_entity.AlertEntity {
	return alert_entity.RandomAlert(n)
}

func reader(c *nats_driver.Client, stream *string, wg *sync.WaitGroup) {
	defer wg.Done()
	c.Pull(*stream)
}
