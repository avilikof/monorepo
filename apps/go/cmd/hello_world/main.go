package main

import (
	"fmt"
	"time"
)

func main() {
	fmt.Println("Starting the program..")
	var n uint64
	startTime := time.Now()
	for n < 1_000_000_000 {
		n += 1
	}
	fmt.Println(time.Since(startTime))
}
