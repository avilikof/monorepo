package main

import (
	"errors"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"time"
)

type MeteoUrl int

const (
	BaseUrl MeteoUrl = iota
	PlacesUrl
	ForecastsUrl
	ForecastsLongUrl
)

func (mu MeteoUrl)GetUrl(place *string) (string, error) {
	switch mu {
	case BaseUrl:
		return "https://api.meteo.lt/v1", nil
	case PlacesUrl:
		return "https://api.meteo.lt/v1/places", nil
	case ForecastsUrl:
		if place == nil {
			return "", fmt.Errorf("place must not be nil")
		}
		base, _ := PlacesUrl.GetUrl(nil)
		fullPlace := fmt.Sprintf("%s/%s/forecasts", base,*place)
		return fullPlace, nil
	case ForecastsLongUrl:
		forecastsUrl, err := ForecastsUrl.GetUrl(place)
		if err != nil {
			return "", err
		}
		fullUrl := fmt.Sprintf("%s/%s/long-term", forecastsUrl,*place)
		return fullUrl, nil
	default:
		return "", fmt.Errorf("something went wrong")
	}
}


func GetResponse(url string) ([]byte, error) {
	slog.Info("Starting scraping..")
	startTime := time.Now()
	resp, err := http.Get(url)
	if err != nil {
		// Handle error if request fails
		fmt.Println("Error:", err)
		return nil, err
	}
	defer resp.Body.Close()

	duration := time.Since(startTime)

	switch resp.StatusCode {
	case http.StatusOK:
		slog.Info("Finished scraping", "duration", duration)
		return getBody(resp)

	case http.StatusTooManyRequests:
		tooManyRequestsCooldown(60)
		return GetResponse(url)
	
	case http.StatusNotFound:
		slog.Warn("page not found", "url", url)
		slog.Info("Finished scraping", "duration", duration)
		return nil, errors.New("404")

	default:
		slog.Error(
			"failed to proceed with", "URL",
			url,
		)
		return nil, fmt.Errorf("error: got status code %d", resp.StatusCode)
	}
}

func tooManyRequestsCooldown(cooldownDuration uint16) {
		slog.Warn("Too many request, sleeping for 1 minute")
		go timeLeft(cooldownDuration)
		time.Sleep(time.Minute)
}
func timeLeft(timeSeconds uint16) {
	for timeSeconds > 0 {
		slog.Info("Resume", "in", timeSeconds)
		timeSeconds -= 10
		time.Sleep(time.Duration(time.Second * 10))
	}
}
func getBody(resp *http.Response) ([]byte, error) {
	return io.ReadAll(resp.Body)
}