package natslistener

import (
	"fmt"
	natsdriver "go-mono/pkg/nats_driver"
	"log"

	"github.com/nats-io/nats.go"
)

func Listen(url, subject string) error {
	nc, _err := natsdriver.NewNatsConnection(&url)
	defer nc.Close()
	if _err != nil {
		return _err
	}
	ps := natsdriver.NewPubSub(nc)

	_err = ps.Subscribe(subject, printMessage)
	if _err != nil {
		log.Fatalf("Error subscribing to subject: %v", _err)
	}
	return nil
}

func printMessage(msg *nats.Msg) {
	fmt.Printf("Received message on subject %s: %s\n", msg.Subject, string(msg.Data))
}

func getData(msg *nats.Msg) []byte {
	return msg.Data
}
