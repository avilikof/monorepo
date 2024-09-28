package alert_entity

import (
	"encoding/json"
	"strconv"
	"testing"
	"time"
)

func TestNewAlertEntity(t *testing.T) {
	alert := NewAlertEntity()
	if alert == nil {
		t.Error("NewAlertEntity should return a non-nil pointer")
	}
}

func TestCreateAlertEntity(t *testing.T) {
	occurrenceId := "123"
	description := "Test alert"
	state := "firing"
	alertId := "456"
	timestamp := time.Now().UnixNano()

	alert := CreateAlertEntity(occurrenceId, description, state, alertId, timestamp)

	if alert.OccurrenceId != occurrenceId {
		t.Errorf("Expected OccurrenceId %s, got %s", occurrenceId, alert.OccurrenceId)
	}
	if alert.Description != description {
		t.Errorf("Expected Description %s, got %s", description, alert.Description)
	}
	if alert.State != state {
		t.Errorf("Expected State %s, got %s", state, alert.State)
	}
	if alert.AlertId != alertId {
		t.Errorf("Expected AlertId %s, got %s", alertId, alert.AlertId)
	}
	if alert.Timestamp != timestamp {
		t.Errorf("Expected Timestamp %d, got %d", timestamp, alert.Timestamp)
	}
}

func TestNewAlertEntityFromBytes(t *testing.T) {
	alert := CreateAlertEntity("123", "Test alert", "firing", "456", time.Now().UnixNano())
	data, _ := json.Marshal(alert)

	newAlert, _err := NewAlertEntityFromBytes(data)
	if _err != nil {
		t.Errorf("Unexpected error: %v", _err)
	}

	if *newAlert != alert {
		t.Error("Unmarshaled alert does not match original")
	}

	_, _err = NewAlertEntityFromBytes([]byte("invalid json"))
	if _err == nil {
		t.Error("Expected error for invalid JSON, got nil")
	}

	_, _err = NewAlertEntityFromBytes([]byte("{}"))
	if _err == nil || _err.Error() != "empty alert" {
		t.Error("Expected 'empty alert' error for empty JSON object")
	}
}

func TestRandomAlert(t *testing.T) {
	alert := RandomAlert(10)

	if alert.OccurrenceId != "" {
		t.Error("RandomAlert should have empty OccurrenceId")
	}
	if alert.Description != "random alert" {
		t.Error("RandomAlert should have 'random alert' as Description")
	}
	if alert.State != "firing" && alert.State != "resolved" {
		t.Error("RandomAlert should have either 'firing' or 'resolved' as State")
	}
	alertId, _err := strconv.ParseInt(alert.AlertId, 10, 64)
	if _err != nil || alertId < 1 || alertId > 10 {
		t.Error("RandomAlert should have AlertId between 1 and 10")
	}
}

func TestAlertEntityMethods(t *testing.T) {
	alert := CreateAlertEntity("123", "Test alert", "firing", "456", time.Now().UnixNano())

	if alert.GetOccurrenceId() != "123" {
		t.Error("GetOccurrenceId failed")
	}
	if alert.GetDescription() != "Test alert" {
		t.Error("GetDescription failed")
	}
	if alert.GetState() != "firing" {
		t.Error("GetState failed")
	}
	if alert.GetAlertId() != "456" {
		t.Error("GetAlertId failed")
	}

	_err := alert.SetDescription("New description")
	if _err != nil || alert.Description != "New description" {
		t.Error("SetDescription failed")
	}

	_err = alert.SetOccurrenceId("789")
	if _err == nil {
		t.Error("SetOccurrenceId should fail when OccurrenceId is already set")
	}

	_err = alert.SetState("resolved")
	if _err != nil || alert.State != "resolved" {
		t.Error("SetState failed")
	}

	_err = alert.SetState("invalid")
	if _err == nil {
		t.Error("SetState should fail with invalid state")
	}
}

func TestAlertEntityToByte(t *testing.T) {
	alert := CreateAlertEntity("123", "Test alert", "firing", "456", time.Now().UnixNano())

	data, _err := alert.ToByte()
	if _err != nil {
		t.Errorf("Unexpected error: %v", _err)
	}

	var newAlert AlertEntity
	_err = json.Unmarshal(data, &newAlert)
	if _err != nil {
		t.Errorf("Failed to unmarshal: %v", _err)
	}

	if newAlert != alert {
		t.Error("Marshaled and unmarshaled alert does not match original")
	}

	emptyAlert := NewAlertEntity()
	_, _err = emptyAlert.ToByte()
	if _err == nil {
		t.Error("ToByte should return error for empty alert")
	}
}

func TestAlertIsEmpty(t *testing.T) {
	emptyAlert := NewAlertEntity()
	if !alertIsEmpty(emptyAlert) {
		t.Error("alertIsEmpty should return true for empty alert")
	}

	nonEmptyAlert := CreateAlertEntity("123", "Test alert", "firing", "456", time.Now().UnixNano())
	if alertIsEmpty(&nonEmptyAlert) {
		t.Error("alertIsEmpty should return false for non-empty alert")
	}
}
