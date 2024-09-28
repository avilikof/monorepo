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
	var _err error
	_err = kc.config.SetKey("group.id", kafka.ConfigValue("myGroup"))
	if _err != nil {
		return _err
	}
	_err = kc.config.SetKey("go.application.rebalance.enable", kafka.ConfigValue(true))
	if _err != nil {
		return _err
	}
	_err = kc.config.SetKey("enable.auto.commit", kafka.ConfigValue(false))
	if _err != nil {
		return _err
	}

	cons, _err := kc.getConsumer()
	if _err != nil {
		panic(_err)
	}

	_err = cons.SubscribeTopics([]string{"test"}, nil)

	defer cons.Close()
	defer close(messageChannel)

	if _err != nil {
		panic(_err)
	}

	for {
		msg, _err := cons.ReadMessage(time.Second)
		if _err != nil {
			if _err.(kafka.Error).IsTimeout() {
				fmt.Println("Reached end for messages...")
				break
			}
			fmt.Printf("Consumer error: %v (%v)\n", _err, msg)
			fmt.Printf("Error code: %s\n", _err.(kafka.Error).IsTimeout())
			return _err
		}
		messageChannel <- msg.Value
	}
	return nil
}

func (kc *KafkaClient) getConsumer() (*kafka.Consumer, error) {
	return kafka.NewConsumer(kc.config)
}
