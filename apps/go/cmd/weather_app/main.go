package main

import (
	"encoding/json"
	"errors"
	"fmt"
	kafkadriver "go-mono/pkg/kafka_driver"
	"io"
	"net/http"
)

type Coordinates struct {
	Latitude  float64 `json:"latitude"`
	Longitude float64 `json:"longitude"`
}

// Define the main structure for the response
type Place struct {
	Code                   string      `json:"code"`
	Name                   string      `json:"name"`
	AdministrativeDivision string      `json:"administrativeDivision"`
	CountryCode            string      `json:"countryCode"`
	Coordinates            Coordinates `json:"coordinates"`
}

func main() {
	// The URL to send the GET request to
	const urlPlaces string = "https://api.meteo.lt/v1/places"
	places := make([]Place, 0, 3000)

	resp, err := getResponse(urlPlaces)
	if err != nil {
		panic(err)
	}

	// Perform the HTTP GET request
	defer resp.Body.Close() // Ensure the response body is closed

	// Read the response body
	body, err := getBody(resp)

	if err != nil {
		fmt.Println("Error reading response:", err)
		return
	}

	// Write data to kafka

	err = json.Unmarshal([]byte(body), &places)
	if err != nil {
		panic(err)
	}

	// Print the response body
	for _, a := range places {
		fmt.Printf("Code: %s\n", a.Code)
		err = writeToKakfa([]byte(string(a.Code)))
		if err != nil {
			panic(err)
		}
	}
}

func getResponse(url string) (*http.Response, error) {
	resp, err := http.Get(url)
	if err != nil {
		// Handle error if request fails
		fmt.Println("Error:", err)
		return nil, err
	}

	// Check if the response status is 200 OK
	if resp.StatusCode != http.StatusOK {
		return nil, errors.New(fmt.Sprintf("Error: got status code %d\n", resp.StatusCode))
	}

	return resp, nil
}

func getBody(resp *http.Response) ([]byte, error) {
	return io.ReadAll(resp.Body)
}

func writeToKakfa(payload []byte) error {
	kafkaHandler := kafkadriver.NewKafkaHandler("192.168.32.161")
	return kafkaHandler.Push([]byte("weather"), payload, "test")
}
