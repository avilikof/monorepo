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
	client, err := opts.Connect()
	if err != nil {
		panic(err)
	}
	return &Client{
		client: client,
	}
}

func (c *Client) Push(subject *string, data []byte) error {
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

func (c *Client) Pull(subj string) {
	// Subscribe to the subject "updates"
	sub, err := c.client.Subscribe("updates", func(msg *nats.Msg) {
		fmt.Printf("Received message: %s\n", string(msg.Data))
	})
	if err != nil {
		log.Fatal(err)
	}
	defer sub.Unsubscribe()

	// Keep the connection alive to continue receiving messages
	select {}
}
