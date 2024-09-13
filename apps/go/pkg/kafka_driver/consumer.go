package kafkadriver

import (
	"fmt"
	"sync"
	"time"

	"github.com/confluentinc/confluent-kafka-go/v2/kafka"
)

type KafkaClient struct {
	config *kafka.ConfigMap
}

func NewKafkaClient(bootstrapAddr string) (*KafkaClient, error) {
	config := &kafka.ConfigMap{
		"bootstrap.servers": bootstrapAddr,
		"auto.offset.reset": "earliest",
	}
	return &KafkaClient{
		config: config,
	}, nil
}

func (kc *KafkaClient) GetMessage(messageChannel chan []byte, wg *sync.WaitGroup) error {
	defer wg.Done()
	var err error
	err = kc.config.SetKey("group.id", kafka.ConfigValue("myGroup"))
	if err != nil {
		return err
	}
	err = kc.config.SetKey("go.application.rebalance.enable", kafka.ConfigValue(true))
	if err != nil {
		return err
	}
	err = kc.config.SetKey("enable.auto.commit", kafka.ConfigValue(false))
	if err != nil {
		return err
	}

	cons, err := kc.getConsumer()
	if err != nil {
		panic(err)
	}

	err = cons.SubscribeTopics([]string{"test"}, nil)

	defer cons.Close()
	defer close(messageChannel)

	if err != nil {
		panic(err)
	}

	for {
		msg, err := cons.ReadMessage(time.Second)
		if err != nil {
			if err.(kafka.Error).IsTimeout() {
				fmt.Println("Reached end for messages...")
				break
			}
			fmt.Printf("Consumer error: %v (%v)\n", err, msg)
			fmt.Printf("Error code: %s\n", err.(kafka.Error).IsTimeout())
			return err
		}
		messageChannel <- msg.Value
	}
	return nil
}

func (kc *KafkaClient) getConsumer() (*kafka.Consumer, error) {
	return kafka.NewConsumer(kc.config)
}
