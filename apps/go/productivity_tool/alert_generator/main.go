package main

import (
	"fmt"
	"github.com/avilikof/monorepo/lib/go/alert_entity"
	"github.com/avilikof/monorepo/lib/go/env_var"
	"github.com/avilikof/monorepo/lib/go/kafka_driver"
	"log"
)

func main() {
	println("Hello World")
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
}

func checkKafka(envVars *env_var.EnvironmentVarHandler) {
	client := kafka_driver.NewKafkaHandler(envVars)
	err := client.Subscribe(envVars)
	if err != nil {
		log.Println(err)
		return
	}
	fmt.Println(client.SubscriptionIsActive())
	for i := 0; true; i++ {
		msg, err := client.Get()
		if err != nil {
			log.Println(err)
			break
		}
		if i%1000 == 0 {
			log.Println(string(msg.Value), i)
		}
	}
}
