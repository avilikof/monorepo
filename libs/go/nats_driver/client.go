package nats_driver

import (
	"fmt"
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

func (c *Client) checkClient() {
	status := c.client.Status()
	fmt.Println(status.String())
}
