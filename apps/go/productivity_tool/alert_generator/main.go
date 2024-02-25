package main

import (
	"fmt"
	"github.com/avilikof/monorepo/lib/go/alert_entity"
	"github.com/avilikof/monorepo/lib/go/env_var"
	"github.com/avilikof/monorepo/lib/go/kafka_driver"
	"log"
	"strconv"
	"sync"
	"time"
)

func main() {
	println("Main started..")
	configurator, _err := env_var.NewEnvVarHandler()
	if _err != nil {
		fmt.Println(_err)
	}
	_err = configurator.LoadDotEnv(".env")
	if _err != nil {
		fmt.Println(_err)
	}

	var wg sync.WaitGroup
	wg.Add(1)
	client := kafka_driver.NewKafkaHandler(&configurator)
	n := uint64(0)
	go func(wg *sync.WaitGroup, k kafka_driver.KafkaHandler) {
		log.Println("Start sending random alerts to stream")
		defer wg.Done()
		for {
			largestNumberStr, err := configurator.Get("LARGEST_NUMBER")
			largestNumber, err := strconv.ParseInt(largestNumberStr, 10, 64)
			alert := alert_entity.RandomAlert(largestNumber)
			n += 1
			if n%1000 == 0 {
				log.Printf("%v alerts produced\n", n)
			}
			_err := produceRandomAlert(&client, &alert)
			if _err != nil {
				log.Println(_err)
				break
			}
			interval, err := configurator.Get("GENERATOR_INTERVAL")
			if err != nil {
				log.Println(err)
			}
			val, err := strconv.ParseInt(interval, 10, 64)
			time.Sleep(time.Duration(val) * time.Millisecond)
		}
	}(&wg, client)
	wg.Wait()
}

func produceRandomAlert(kafkaHandler *kafka_driver.KafkaHandler, randomAlert *alert_entity.AlertEntity) error {
	key := randomAlert.GetAlertId()
	alertBytes, _err := randomAlert.ToByte()
	if _err != nil {
		return _err
	}
	return kafkaHandler.Push([]byte(key), alertBytes, "alerts")
}
