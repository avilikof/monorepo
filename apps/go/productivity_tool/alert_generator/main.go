package main

import (
	"fmt"
	"github.com/avilikof/monorepo/lib/go/alert_entity"
	"github.com/avilikof/monorepo/lib/go/env_var"
	"github.com/avilikof/monorepo/lib/go/kafka_driver"
	"log"
	"sync"
	"time"
)

func main() {
	println("Main started..")
	alert := alert_entity.RandomAlert()
	fmt.Printf("%+v\n", alert)
	configurator, err := env_var.NewEnvVarHandler()
	if err != nil {
		fmt.Println(err)
	}
	err = configurator.LoadDotEnv(".env")
	if err != nil {
		fmt.Println(err)
	}

	var wg sync.WaitGroup
	wg.Add(1)
	client := kafka_driver.NewKafkaHandler(&configurator)
	n := uint64(0)
	go func(wg *sync.WaitGroup, k kafka_driver.KafkaHandler, a alert_entity.AlertEntity) {
		log.Println("Start sending random alerts to stream")
		defer wg.Done()
		for {
			n += 1
			if n%1000 == 0 {
				log.Printf("%v alerts produced\n", n)
			}
			err := generateRandomAlert(&client, &alert)
			if err != nil {
				log.Println(err)
				break
			}
			time.Sleep(100 * time.Millisecond)
		}
	}(&wg, client, alert)
	wg.Wait()
}

func generateRandomAlert(kafkaHandler *kafka_driver.KafkaHandler, randomAlert *alert_entity.AlertEntity) error {
	alertBytes, err := randomAlert.ToByte()
	if err != nil {
		return err
	}
	return kafkaHandler.Push([]byte("alert"), []byte(alertBytes), "alerts")
}
