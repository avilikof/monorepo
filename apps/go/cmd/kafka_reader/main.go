package main

import (
	"fmt"
	kafkadriver "go-mono/pkg/kafka_driver"
	"sync"
)

func main() {

	// var messageChan = make(chan []byte)
	var wg sync.WaitGroup

	wg.Add(2)
	// kafkaClient, err := kafkadriver.NewKafkaClient("192.168.32.161")
	kh := kafkadriver.NewKafkaHandler("192.168.32.161")

	kh.Subscribe("test")
	for {
		msg, err := kh.Get()
		if err != nil {
			panic(err)
		}
		fmt.Printf("Message :: %s\n", msg)
	}
	// 	fmt.Println(msg)
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
