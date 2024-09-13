package natslistener

import (
	"fmt"
	natsdriver "go-mono/pkg/nats_driver"
	"log"

	"github.com/nats-io/nats.go"
)

func Listen(url, subject string) error {
	nc, err := natsdriver.NewNatsConnection(&url)
	defer nc.Close()
	if err != nil {
		return err
	}
	ps := natsdriver.NewPubSub(nc)

	err = ps.Subscribe(subject, printMessage)
	if err != nil {
		log.Fatalf("Error subscribing to subject: %v", err)
	}
	return nil
}

func printMessage(msg *nats.Msg) {
	fmt.Printf("Received message on subject %s: %s\n", msg.Subject, string(msg.Data))
}

func getData(msg *nats.Msg) []byte {
	return msg.Data
}
