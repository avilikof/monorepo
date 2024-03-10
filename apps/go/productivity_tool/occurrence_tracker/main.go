package main

import (
	"fmt"
	"log"
	"os"

	"github.com/avilikof/monorepo/lib/go/alert_entity"
	"github.com/avilikof/monorepo/lib/go/env_var"
	"github.com/avilikof/monorepo/lib/go/kafka_driver"
)

func main() {
	fmt.Println("Hello there")
	readAlerts()
}

func readAlerts() {
	envVars, err := env_var.NewEnvVarHandler()
	if err != nil {
		log.Println(err)
	}
	err = envVars.LoadDotEnv(".env")
	if err != nil {
		log.Printf("failed to load config: %v\n", err)
		os.Exit(0)
	}
	kafkaClient := kafka_driver.NewKafkaHandler(&envVars)
	occHandler := NewOccurrenceHandler()
	err = kafkaClient.Subscribe(&envVars)
	if err != nil {
		log.Println(err)
		os.Exit(0)
	}
	for {
		err := printMessage(&kafkaClient, &occHandler)
		if err != nil {
			continue
		}
	}
}

func printMessage(kafka *kafka_driver.KafkaHandler, occHandler *OccurrenceHandler) error {
	msg, _err := kafka.Get()
	if _err != nil {
		return _err
	}
	if len(string(msg.Value)) == 0 {
		return fmt.Errorf("empty response")
	}
	alert, _err := alert_entity.NewAlertEntityFromBytes(msg.Value)
	if _err != nil {
		return _err
	}
	_err = occHandler.Handle(*alert)
	if _err != nil {
		return _err
	}
	return nil
}
