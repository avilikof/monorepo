package nats_driver

import (
	"fmt"
	"log"
	"time"

	"github.com/nats-io/nats.go"
)

type Client struct {
	client *nats.Conn
}

func DefaultClient(url *string) *Client {
	opts := nats.Options{
		Url:            *url,
		AllowReconnect: true,
		Timeout:        100 * time.Second,
		Verbose:        true,
	}
	client, _err := opts.Connect()
	if _err != nil {
		panic(_err)
	}
	return &Client{
		client: client,
	}
}

func (c *Client) Push(subject *string, data []byte) error {
	fmt.Println("sending")
	return c.client.Publish(*subject, data)
}

func (c *Client) Close() {
	c.client.Close()
}

func (c *Client) checkClient() {
	status := c.client.Status()
	fmt.Println(status.String())
}

func (c *Client) Info() string {
	return c.client.ConnectedServerId()
}

func (c *Client) Version() string {
	return c.client.ConnectedServerVersion()
}

func (c *Client) Pull(subj string) string {
	fmt.Println("Reading")
	defer fmt.Println("I'm done reading")
	// Subscribe to the subject "updates"
	// subscription, _err := c.client.Subscribe(subj, func(msg *nats.Msg) {
	// 	fmt.Printf("Received message: %s\n", string(msg.Data))
	// })
	s, _err := c.client.SubscribeSync(subj)
	if _err != nil {
		panic(_err)
	}
	m, _err := s.NextMsg(2 * time.Second)
	if _err != nil {
		log.Fatal(_err)
		panic(_err)
	}
	return string(m.Data)
}
