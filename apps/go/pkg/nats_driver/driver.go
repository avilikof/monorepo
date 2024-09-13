package natsdriver

import (
	"errors"
	"time"

	"github.com/nats-io/nats.go"
)

type NatsConnection struct {
	connection *nats.Conn
}

func NewNatsConnection(url *string) (*NatsConnection, error) {
	if url == nil {
		return nil, errors.New("url cannot be nil")
	}
	if len(*url) < 1 {
		return nil, errors.New("url cannot be blank")
	}
	opts := setDefaultNatsOpts(url)
	conn, err := opts.Connect()
	if err != nil {
		return nil, err
	}
	return &NatsConnection{
		connection: conn,
	}, nil

}
func setDefaultNatsOpts(url *string) *nats.Options {
	return &nats.Options{
		Url:            *url,
		AllowReconnect: true,
		Timeout:        100 * time.Second,
		Verbose:        true,
	}
}
func (nc *NatsConnection) Close() {
	nc.connection.Close()
}
