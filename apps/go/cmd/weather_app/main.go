package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"sync"
	"time"
)


func main() {
	ctx := context.Background()

	placesUrl, _ := PlacesUrl.GetUrl(nil)
	places := make([]Place, 0, 3000)

	redisHandler := NewRedisHandler("192.168.32.161:32779")

	body, err := GetResponse(placesUrl)
	if err != nil {
		panic(err)
	}

	err = json.Unmarshal([]byte(body), &places)
	if err != nil {
		panic(err)
	}

	slog.Info("Start writing..")
	startTime := time.Now()
	var wg sync.WaitGroup
	// Print the response body
	for _, a := range places {
		wg.Add(1)
		key := a.Code
		data, err := json.Marshal(a)
		if err != nil {
			panic(err)
		}
		go func (){
			if err := redisHandler.Write(ctx,&wg, key, &data); err != nil {
				panic(err)
			}
		}()
	}
	wg.Wait()
	slog.Info("Finished writing to redis", "time spent", time.Since(startTime))

	numClients := redisHandler.Keys(ctx, "*")
	fmt.Println(redisHandler.DBSize(ctx))
	time.Sleep(5 * time.Second)

	for _, place := range numClients {
		body, err := getForecasts(place)
		if err != nil {
			continue
		}
		shortForcastKey := fmt.Sprintf("short_forcast_%s", place)
		if err := redisHandler.Write(ctx, nil, shortForcastKey, &body); err != nil {
			panic(err)
		}
	}
}


func getForecasts(code string) ([]byte, error) {
	url, err := ForecastsUrl.GetUrl(&code)
	if err != nil {
		panic(err)
	}
	return GetResponse(url)
}
