package main

import (
	"fmt"
	"github.com/avilikof/monorepo/lib/go/alert_entity"
)

func main() {
	println("Hello World")
	alert := alert_entity.RandomAlert()
	fmt.Printf("%+v", alert)
}
