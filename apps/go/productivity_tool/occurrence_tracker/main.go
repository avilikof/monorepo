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
	envVars, _err := env_var.NewEnvVarHandler()
	if _err != nil {
		log.Println(_err)
	}
	_err = envVars.LoadDotEnv(".env")
	if _err != nil {
		log.Printf("failed to load config: %v\n", _err)
		os.Exit(0)
	}
	kafkaClient := kafka_driver.NewKafkaHandler(&envVars)
	occHandler := NewOccurrenceHandler()
	_err = kafkaClient.Subscribe(&envVars)
	if _err != nil {
		log.Println(_err)
		os.Exit(0)
	}
	for {
		_err := printMessage(&kafkaClient, &occHandler)
		if _err != nil {
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
