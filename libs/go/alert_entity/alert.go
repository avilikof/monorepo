package alert_entity

import (
	"crypto/rand"
	"encoding/json"
	"log"
	"math/big"
	"strconv"
	"time"
)

type AlertEntity struct {
	occurrenceId string
	timestamp    int64
	description  string
	state        string
	alertId      string
}

func NewAlertEntity() AlertEntity {
	return AlertEntity{}
}
func CreateAlertEntity(occurrenceId, description, state, alertId string, timestamp int64) AlertEntity {
	return AlertEntity{
		occurrenceId: occurrenceId,
		timestamp:    timestamp,
		description:  description,
		state:        state,
		alertId:      alertId,
	}
}
func RandomAlert() AlertEntity {
	return AlertEntity{
		occurrenceId: "",
		timestamp:    time.Now().UnixNano(),
		description:  "random alert",
		state:        getRandomState(),
		alertId:      strconv.FormatInt(getRandomLetter(), 10),
	}
}

func (a *AlertEntity) GetOccurrenceId() string {
	return a.occurrenceId
}
func (a *AlertEntity) GetTimestamp() int64 {
	return a.timestamp
}
func (a *AlertEntity) GetDescription() string {
	return a.description
}
func (a *AlertEntity) GetState() string {
	return a.state
}
func (a *AlertEntity) GetAlertId() string {
	return a.alertId
}
func (a *AlertEntity) ToByte() ([]byte, error) {
	return json.Marshal(a)
}

func getRandomLetter() int64 {
	biggestNumber := big.NewInt(10) // Convert biggestNumber value to big.Int
	number, err := rand.Int(rand.Reader, biggestNumber)
	if err != nil {
		log.Printf("error getting random number: %v", err)
		return 0
	}
	return number.Int64() + 1 // Add 1 to get 1-10 range
}

func getRandomState() string {
	state := []string{"firing", "resolved"}
	biggestNumber := big.NewInt(int64(len(state)))
	num, err := rand.Int(rand.Reader, biggestNumber)
	if err != nil {
		log.Printf("error getting random nuber: %v", err)
	}
	return state[num.Int64()]
}
