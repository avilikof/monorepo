package main

import (
	"fmt"
	advent "go-mono/internal/advent_of_code_2023"
	"strings"
)

func main() {
	const DAY_1 = "data/advent_of_code_2023/input_day_1.txt"
	fmt.Println("Hello world")
	fmt.Println(advent.One.AsString())
	fileContent, _err := advent.ReadFile(DAY_1)
	calibrationDocument := strings.Split(string(fileContent), "\n")
	if _err != nil {
		panic(_err)
	}
	calibrationHandler := advent.NewCalibrationHandler(calibrationDocument)
	calibrationValue := calibrationHandler.GetCalibrationValue()
	fmt.Println(calibrationValue)
}
