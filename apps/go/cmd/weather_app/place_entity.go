package main

import "math/rand"


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

// Default returns a Place with random values.
func PlaceDefault() Place {
	return Place{
		Code:                   randomString(5),
		Name:                   randomString(10),
		AdministrativeDivision: randomString(10),
		CountryCode:            randomString(2),
		Coordinates: Coordinates{
			Latitude:  randomFloat(-90.0, 90.0),
			Longitude: randomFloat(-180.0, 180.0),
		},
	}
}

// Helper function to generate a random string of a given length.
func randomString(n int) string {
	const letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	b := make([]byte, n)
	for i := range b {
		b[i] = letters[rand.Intn(len(letters))]
	}
	return string(b)
}

// Helper function to generate a random float64 between min and max.
func randomFloat(min, max float64) float64 {
	return min + rand.Float64()*(max-min)
}