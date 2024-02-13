package alert_entity

import (
	"crypto/rand"
	"encoding/json"
	"fmt"
	"log"
	"math/big"
	"strconv"
	"time"
)

type AlertEntity struct {
	OccurrenceId string `json:"occurrenceId"`
	Timestamp    int64  `json:"timestamp"`
	Description  string `json:"description"`
	State        string `json:"state"`
	AlertId      string `json:"alertId"`
}

func NewAlertEntity() *AlertEntity {
	return &AlertEntity{}
}
func CreateAlertEntity(occurrenceId, description, state, alertId string, timestamp int64) AlertEntity {
	return AlertEntity{
		OccurrenceId: occurrenceId,
		Timestamp:    timestamp,
		Description:  description,
		State:        state,
		AlertId:      alertId,
	}
}

func NewAlertEntityFromBytes(data []byte) (*AlertEntity, error) {
	var entity AlertEntity
	err := json.Unmarshal(data, &entity)
	if err != nil {
		return nil, err
	}
	if entity == *NewAlertEntity() {
		return nil, fmt.Errorf("empty alert")
	}
	return &entity, nil
}

//func main() {
//	// Example JSON data
//	jsonData := []byte(`{
//		"occurrenceId": "12345",
//		"timestamp": 161803398874989,
//		"description": "Example alert description",
//		"state": "active",
//		"alertId": "alert123"
//	}`)
//
//	// Create an AlertEntity from []byte
//	alertEntity, err := NewAlertEntityFromBytes(jsonData)
//	if err != nil {
//		log.Fatalf("Error creating AlertEntity from bytes: %v", err)
//	}
//
//	fmt.Printf("AlertEntity: %+v\n", alertEntity)
//}

func RandomAlert() AlertEntity {
	return AlertEntity{
		OccurrenceId: "",
		Timestamp:    time.Now().UnixNano(),
		Description:  "random alert",
		State:        getRandomState(),
		AlertId:      strconv.FormatInt(getRandomLetter(), 10),
	}
}

func (a *AlertEntity) GetOccurrenceId() string {
	return a.OccurrenceId
}
func (a *AlertEntity) GetTimestamp() int64 {
	return a.Timestamp
}
func (a *AlertEntity) GetDescription() string {
	return a.Description
}
func (a *AlertEntity) GetState() string {
	return a.State
}
func (a *AlertEntity) GetAlertId() string {
	return a.AlertId
}
func (a *AlertEntity) ToByte() ([]byte, error) {
	if *a == *NewAlertEntity() {
		return nil, fmt.Errorf("alert contains no data")
	}
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

func alertIsEmpty(alert *AlertEntity) bool {
	return *alert == AlertEntity{}
}
