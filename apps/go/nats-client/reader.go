package main

import (
	"github.com/avilikof/monorepo/libs/go/nats_driver"
)

type NatsReaderClient struct {
	natsServerAddr string
}

func NewNatsReaderClient(url string) *NatsReaderClient {
	return &NatsReaderClient{
		natsServerAddr: url,
	}
}
func (nc *NatsReaderClient) connect_to_server() (nats_driver.Client, error) {
	return *nats_driver.DefaultClient(&nc.natsServerAddr), nil
}
func (nc *NatsReaderClient) Pull(subject string) (string, error) {
	client, err := nc.connect_to_server()
	if err != nil {
		panic(err)
	}
	return client.Pull(subject), nil
}