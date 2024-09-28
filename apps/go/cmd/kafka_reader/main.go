package main

import (
	"fmt"
	kafkadriver "go-mono/pkg/kafka_driver"
	"sync"
	"time"
)

func main() {

	// var messageChan = make(chan []byte)
	var wg sync.WaitGroup

	wg.Add(2)
	// kafkaClient, _err := kafkadriver.NewKafkaClient("192.168.32.161")
	kh := kafkadriver.NewKafkaHandler("192.168.32.161")
	kh.SetConfigValues("enble.auto.commit", false)

	kh.Subscribe("test")
	startTime := time.Now()
	for {
		msg, _err := kh.Get()
		if _err != nil {
			fmt.Println(_err)
			break
		}
		fmt.Printf("Message :: %s\n", &msg.Value)
	}
	timeElapsed := time.Since(startTime) // TODO: log how much time it needed to read messages.
	fmt.Println(timeElapsed)
	// }
	// go kafkaClient.GetMessage(messageChan, &wg)
	// go func() {
	// 	defer wg.Done()
	// 	for {
	// 		msg := <-messageChan
	// 		if len(msg) == 0 {
	// 			break
	// 		}
	// 		fmt.Printf("Message :: %s\n", string(msg))
	// 	}
	// }()
	// wg.Wait()

}
