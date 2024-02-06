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
	value, _ := configurator.Get("APP_TEST")
	valueTwo, _ := configurator.Get("TEST")
	fmt.Println(value)
	fmt.Println(valueTwo)

	checkKafka(&configurator)

	var wg sync.WaitGroup
	wg.Add(1)
	client := kafka_driver.NewKafkaHandler(&configurator)
	go func(wg *sync.WaitGroup, k kafka_driver.KafkaHandler, a alert_entity.AlertEntity) {
		defer wg.Done()
		for {
			err := randomAlertGeneration(&client, &alert)
			if err != nil {
				log.Println(err)
				break
			}
			time.Sleep(100 * time.Millisecond)
		}
	}(&wg, client, alert)
}

func randomAlertGeneration(kafkaHandler *kafka_driver.KafkaHandler, randomAlert *alert_entity.AlertEntity) error {
	alertBytes, err := randomAlert.ToByte()
	if err != nil {
		return err
	}
	return kafkaHandler.Push([]byte("alert"), []byte(alertBytes), "alerts")
}

func checkKafka(envVars *env_var.EnvironmentVarHandler) {
	client := kafka_driver.NewKafkaHandler(envVars)
	err := client.Subscribe(envVars)
	if err != nil {
		log.Println(err)
		return
	}
	fmt.Println(client.SubscriptionIsActive())
	var wg sync.WaitGroup
	wg.Add(2)
	go func(wg *sync.WaitGroup) {
		defer wg.Done()
		for i := 0; true; i++ {
			msg, err := client.Get()
			if err != nil {
				log.Println(err, i)
				break
			}
			if string(msg.Key) == "test" {
				log.Printf("print by key: %v\n", string(msg.Value))
			}
			if i%1000 == 0 {
				log.Printf("print by sequence: %v :: %v\n", string(msg.Value), i)
			}
		}
	}(&wg)

	key := "test"
	value := "this is something awesome and working, what is awesome"
	topic, _ := envVars.Get("KAFKA_TOPIC")
	go func(wg *sync.WaitGroup) {
		defer wg.Done()
		err := client.Push([]byte(key), []byte(value), topic)
		if err != nil {
			fmt.Println(err)
		}
	}(&wg)
	wg.Wait()
}
