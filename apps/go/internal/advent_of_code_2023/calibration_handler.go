package adventofcode2023

import (
	"fmt"
	"os"
	"strconv"
	"unicode"
)

type (
	ExecuteOnIteration func(string) []uint8
	Calibrator         struct {
		document []string
		value    uint
	}
)

func NewCalibrationHandler(calibrationDocument []string) *Calibrator {
	return &Calibrator{
		calibrationDocument,
		0,
	}
}

func (c *Calibrator) ReadByLine(execute ExecuteOnIteration) {
	for _, line := range c.document {
		execute(line)
	}
}

func (c *Calibrator) calculateCalibrationValues() {
	numsFromAllLines := getNumsFromAllLines(c.document)
	calibrationNumbers := getCalibrationNumbers(numsFromAllLines)
	c.value = sumIntsFromArray(calibrationNumbers)
}

func (c *Calibrator) GetCalibrationValue() uint {
	c.calculateCalibrationValues()
	return c.value
}

func sumIntsFromArray(calibrationNumbers []uint) uint {
	var cn uint
	for _, n := range calibrationNumbers {
		cn += uint(n)
	}
	return cn
}

func getCalibrationNumbers(numsFromAllLines [][]uint) []uint {
	var calibrationNumbers []uint
	for _, num := range numsFromAllLines {
		calibrationNumber := joinTwoNumber(num[0], num[len(num)-1])
		calibrationNumbers = append(calibrationNumbers, calibrationNumber)
	}
	return calibrationNumbers
}

func getNumsFromAllLines(calibrationDocument []string) [][]uint {
	var numbersFromAllLines [][]uint
	for _, line := range calibrationDocument {
		numbersFromLine := getNumbersFromString(line)
		if len(numbersFromLine) > 0 {
			numbersFromAllLines = append(numbersFromAllLines, numbersFromLine)
		}
	}
	return numbersFromAllLines
}

func getNumbersFromString(s string) []uint {
	var nums []uint
	for _, r := range s {
		if unicode.IsDigit(r) {
			n, err := to_uint8(string(r))
			if err != nil {
				panic(err)
			}
			nums = append(nums, n)
		}
		checkIfWordIsNumber(r)
	}
	return nums
}

func checkIfWordIsNumber(line rune) {
	if line == 'o' {
		fmt.Println(string(line))
	}
}

func ReadFile(path string) ([]byte, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return data, err
	}
	return data, nil
}

// CalibrationNumbers are first and last number from the line
// in a maner: a; b; calibrationNumber = ab
func joinTwoNumber(a, b uint) uint {
	return a*10 + b
}

func to_uint8(numberAsString string) (uint, error) {
	value, err := strconv.ParseUint(numberAsString, 10, 8)
	if err != nil {
		return 0, err
	}
	return uint(value), nil
}
