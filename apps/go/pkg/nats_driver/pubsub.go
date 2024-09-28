package natsdriver

import (
	"github.com/nats-io/nats.go"
	"github.com/rs/zerolog/log"
)

type PubSub struct {
	nc *NatsConnection
}

func NewPubSub(connection *NatsConnection) *PubSub {
	return &PubSub{
		connection,
	}
}

func (ps *PubSub) Publish(subject string, data []byte) error {
	_err := ps.nc.connection.Publish(subject, data)
	if _err != nil {
		log.Printf("Failed to publish to subject %s: %v", subject, _err)
		return _err
	}
	return nil
}

func (ps *PubSub) Close() {
	ps.nc.connection.Close()
}

func (ps *PubSub) Subscribe(subject string, callback func(msg *nats.Msg)) error {
	_, _err := ps.nc.connection.Subscribe(subject, callback)
	if _err != nil {
		return _err
	}

	select {}
}
