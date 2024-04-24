package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"time"

	"github.com/avilikof/monorepo/lib/go/alert_entity"
	"github.com/avilikof/monorepo/lib/go/nats-driver"
	"github.com/nats-io/nats.go"
	"github.com/nats-io/nats.go/jetstream"
)

func main() {
	// Connect to a server

	url := "nats://192.168.32.161:4222"
	natsClient := nats - driver.Default(&url)
	opts := nats.Options{
		Url:            URL,
		AllowReconnect: true,
		Timeout:        100 * time.Second,
		Verbose:        true,
	}

	nc, err := opts.Connect()
	if err != nil {
		log.Println(err)
	}
	defer func(nc *nats.Conn) {
		err := nc.Drain()
		if err != nil {
			panic(err)
		}
	}(nc)

	js, err := jetstream.New(nc)
	if err != nil {
		log.Println(err)
	}

	cfg := jetstream.StreamConfig{
		Name:     "ALERTS",
		Subjects: []string{"alerts.>"},
		MaxAge:   5 * time.Minute,
	}

	cfg.Storage = jetstream.FileStorage

	streamCtx, cancel := context.WithTimeout(context.Background(), 100*time.Second)
	defer cancel()

	// alertStream, err := js.CreateStream(streamCtx, cfg)
	// if errors.Is(err, jetstream.ErrStreamNameAlreadyInUse) {
	//	alertStream, err = js.UpdateStream(streamCtx, cfg)
	//	if err != nil {
	//		log.Println(err)
	//	}
	// } else if err != nil {
	//	log.Println(err)
	// }

	log.Println("created the stream")

	const N = 1000

	log.Println("Start sending")

	err = syncNatsProducer(0, js, streamCtx)
	if err != nil {
		log.Fatal(err)
	}
	err = asyncNatsProducer(N, js)
	if err != nil {
		log.Fatal(err)
	}
	log.Println("Finished sending to JetStream")

	// const MSG_COUNT = 100000000
	// cfg.MaxMsgs = MSG_COUNT
	// _, err = js.UpdateStream(streamCtx, cfg)
	// if err != nil {
	// 	log.Println(err)
	// }
	// fmt.Printf("set max messages to %v\n", MSG_COUNT)

	// printStreamState(streamCtx, alertStream)

	// cfg.MaxBytes = 300000
	// _, err = js.UpdateStream(streamCtx, cfg)
	// if err != nil {
	// 	log.Println(err)
	// }
	// fmt.Println("set max bytes to 300")

	// printStreamState(streamCtx, alertStream)

	// const MAX_TIME = 600
	// cfg.MaxAge = MAX_TIME * time.Second
	// _, err = js.UpdateStream(streamCtx, cfg)
	// if err != nil {
	//	log.Println(err)
	// }
	// fmt.Printf("set max age to %v seconds\n", MAX_TIME)

	// printStreamState(streamCtx, alertStream)

	// fmt.Println("sleeping one second...")
	// time.Sleep(time.Second)

	// printStreamState(streamCtx, alertStream)

}

func printStreamState(ctx context.Context, stream jetstream.Stream) {
	info, _ := stream.Info(ctx)
	b, _ := json.MarshalIndent(info.State, "", " ")
	fmt.Println("inspecting stream info")
	fmt.Println(string(b))
}

func asyncNatsProducer(numberOfMessages uint32, js jetstream.JetStream) error {

	log.Println("Starting async producer..")
	startTime := time.Now()
	for _ = range numberOfMessages {
		alert, _ := randomAlert()
		_, err := js.PublishAsync("alerts.numbers", alert)
		if err != nil {
			return err
		}
	}

	select {
	case <-js.PublishAsyncComplete():
		log.Printf("published %v messages in %v\n", numberOfMessages, time.Now().Sub(startTime))
	case <-time.After(100 * time.Second):
		log.Fatal("publish took too long")
	}
	// printStreamState(streamCtx, alertStream)

	log.Printf("Finished async sending to JetStream, took: %v for %v msgs\n", time.Now().Sub(startTime), numberOfMessages)
	return nil
}

func syncNatsProducer(numOfMessages uint32, js jetstream.JetStream, streamCtx context.Context) error {

	log.Println("Starting sync producer..")
	startTime := time.Now()
	for range numOfMessages {
		_, err := js.Publish(streamCtx, "alerts.firing", nil)
		if err != nil {
			return err
		}
	}
	log.Printf("Finished sync sending to JetStream, took: %v for %v msgs\n", time.Now().Sub(startTime), numOfMessages)
	return nil
}

func randomAlert() ([]byte, error) {
	alert := alert_entity.RandomAlert(1000000)
	return alert.ToByte()
}
