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
	_err := json.Unmarshal(data, &entity)
	if _err != nil {
		return nil, _err
	}
	if entity == *NewAlertEntity() {
		return nil, fmt.Errorf("empty alert")
	}
	return &entity, nil
}

func RandomAlert(largestNumber int64) AlertEntity {
	return AlertEntity{
		OccurrenceId: "",
		Timestamp:    time.Now().UnixNano(),
		Description:  "random alert",
		State:        getRandomState(),
		AlertId:      strconv.FormatInt(getRandomLetter(largestNumber), 10),
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
func (a *AlertEntity) SetDescription(desc string) error {
	a.Description = desc
	return nil
}
func (a *AlertEntity) SetOccurrenceId(id string) error {
	if a.OccurrenceId != "" {
		return fmt.Errorf("occurrenceId is set, cannot change")
	}
	a.OccurrenceId = id
	return nil
}
func (a *AlertEntity) SetState(state string) error {
	if state != "firing" && state != "resolved" {
		return fmt.Errorf("unsuported state provided")
	}
	a.State = state
	return nil
}
func (a *AlertEntity) ToByte() ([]byte, error) {
	if *a == *NewAlertEntity() {
		return nil, fmt.Errorf("alert contains no data")
	}
	return json.Marshal(a)
}

func getRandomLetter(largestNumber int64) int64 {
	biggestNumber := big.NewInt(largestNumber) // Convert biggestNumber value to big.Int
	number, _err := rand.Int(rand.Reader, biggestNumber)
	if _err != nil {
		log.Printf("error getting random number: %v", _err)
		return 0
	}
	return number.Int64() + 1 // Add 1 to get 1-10 range
}

func getRandomState() string {
	state := []string{"firing", "resolved"}
	biggestNumber := big.NewInt(int64(len(state)))
	num, _err := rand.Int(rand.Reader, biggestNumber)
	if _err != nil {
		log.Printf("error getting random nuber: %v", _err)
	}
	return state[num.Int64()]
}

func alertIsEmpty(alert *AlertEntity) bool {
	return *alert == AlertEntity{}
}
