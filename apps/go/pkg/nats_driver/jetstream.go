package natsdriver

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/nats-io/nats.go/jetstream"
)

type JS struct {
	nc        *NatsConnection
	jetStream jetstream.JetStream
	ctx       context.Context
	stream    jetstream.Stream
}

func NewJetStream(nc *NatsConnection) (*JS, error) {
	js, _err := jetstream.New(nc.connection)
	if _err != nil {
		return nil, _err
	}
	return &JS{
		nc,
		js,
		nil,
		nil,
	}, nil
}

// Create a stream
func (js *JS) CreateNewStream(subject, streamName string) (*context.CancelFunc, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	js.ctx = ctx

	stream, _err := js.jetStream.CreateStream(ctx, jetstream.StreamConfig{
		Name:     streamName,
		Subjects: []string{subject}},
	)
	if _err != nil {
		cancel()
		fmt.Println("Error creating stream:", _err)
		return nil, _err
	}
	js.stream = stream
	cons, _err := js.stream.CreateOrUpdateConsumer(js.ctx, jetstream.ConsumerConfig{})
	if _err != nil {
		cancel()
		return nil, _err
	}
	js.Publush(subject, []byte("test asd"))
	wg := sync.WaitGroup{}
	wg.Add(3)

	cc, _ := cons.Consume(func(msg jetstream.Msg) {
		msg.Ack()
		fmt.Println("received msg on", msg.Subject())
		wg.Done()
	})
	wg.Wait()

	cc.Stop()
	return &cancel, nil
}

func (js *JS) Publush(subject string, payload []byte) error {
	js.jetStream.Publish(js.ctx, subject, payload)
	return nil
}
