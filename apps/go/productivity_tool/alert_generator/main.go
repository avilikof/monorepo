package main

import (
	"fmt"
	"github.com/avilikof/monorepo/lib/go/alert_entity"
	"github.com/avilikof/monorepo/lib/go/env_var"
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
}
