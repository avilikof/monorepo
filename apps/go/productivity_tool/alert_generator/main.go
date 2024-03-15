package main

import (
	"fmt"
	"github.com/avilikof/monorepo/lib/go/alert_entity"
	"github.com/avilikof/monorepo/lib/go/env_var"
	"github.com/avilikof/monorepo/lib/go/kafka_driver"
	"github.com/nats-io/nats.go"
	"log"
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

	natsClient, err := nats.Connect("192.168.32.163:4222")
	if err != nil {
		fmt.Printf("Err 1:%v\n", err)
	}

	var wg sync.WaitGroup
	wg.Add(1)
	client := kafka_driver.NewKafkaHandler(&configurator)
	n := uint64(0)
	go func() {
		log.Println("Start sending random alerts to stream")
		defer wg.Done()
		startTime := time.Now()
		for {
			//largestNumberStr, err := configurator.Get("LARGEST_NUMBER")
			//largestNumber, err := strconv.ParseInt(largestNumberStr, 10, 64)
			alert := alert_entity.RandomAlert(100000)
			n += 1
			if n%1000 == 0 {

				fmt.Println(time.Since(startTime))
				log.Printf("%v alerts produced\n", n)
				startTime = time.Now()
			}
			_err := produceRandomAlert(&client, &alert)
			produceRandomAlertToNats(natsClient, &alert)
			if _err != nil {
				log.Println(_err)
				break
			}
			time.Sleep(100 * time.Millisecond)
		}
	}()
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

func produceRandomAlertToNats(natsClient *nats.Conn, alert *alert_entity.AlertEntity) {
	alertBytes, _ := alert.ToByte()
	err := natsClient.Publish("alerts", alertBytes)
	if err != nil {
		fmt.Printf("Err 2 %v :\n", err)
	}

}
