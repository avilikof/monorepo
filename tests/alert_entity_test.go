package tests

import (
	"github.com/avilikof/monorepo/lib/go/alert_entity"
	"strconv"
	"testing"
	"time"
)

func TestNewAlertEntity(t *testing.T) {
	alert := alert_entity.NewAlertEntity()
	if alert.GetOccurrenceId() != "" || alert.GetTimestamp() != 0 || alert.GetDescription() != "" || alert.GetState() != "" || alert.GetAlertId() != "" {
		t.Errorf("NewAlertEntity() failed, expected empty AlertEntity, got %+v", alert)
	}
}

func TestRandomAlert(t *testing.T) {
	alert := alert_entity.RandomAlert()
	if alert.GetDescription() != "random alert" {
		t.Errorf("RandomAlert() failed, expected description 'random alert', got %s", alert.GetDescription())
	}

	if alert.GetTimestamp() <= 0 || alert.GetTimestamp() > time.Now().UnixNano() {
		t.Errorf("RandomAlert() failed, timestamp %d is out of expected range", alert.GetTimestamp())
	}

	if alert.GetState() != "firing" && alert.GetState() != "resolved" {
		t.Errorf("RandomAlert() failed, state %s is not expected", alert.GetState())
	}

	alertId, _err := strconv.ParseInt(alert.GetAlertId(), 10, 64)
	if _err != nil || alertId < 1 || alertId > 10 {
		t.Errorf("RandomAlert() failed, alertId %s is out of expected range", alert.GetAlertId())
	}
}

func TestGetters(t *testing.T) {
	alert := alert_entity.CreateAlertEntity(
		"test_id",
		"test description",
		"firing",
		"5",
		123456789,
	)

	if alert.GetOccurrenceId() != "test_id" {
		t.Errorf("GetOccurrenceId() failed, expected 'test_id', got %s", alert.GetOccurrenceId())
	}

	if alert.GetTimestamp() != 123456789 {
		t.Errorf("GetTimestamp() failed, expected 123456789, got %d", alert.GetTimestamp())
	}

	if alert.GetDescription() != "test description" {
		t.Errorf("GetDescription() failed, expected 'test description', got %s", alert.GetDescription())
	}

	if alert.GetState() != "firing" {
		t.Errorf("GetState() failed, expected 'firing', got %s", alert.GetState())
	}

	if alert.GetAlertId() != "5" {
		t.Errorf("GetAlertId() failed, expected '5', got %s", alert.GetAlertId())
	}
}
