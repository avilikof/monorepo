package main

import (
	"fmt"
	"github.com/avilikof/monorepo/lib/go/alert_entity"
	"log"
	"strconv"
	"time"
)

type OccurrenceHandler struct {
	internalStorage map[string][]byte
}

func NewOccurrenceHandler() OccurrenceHandler {
	return OccurrenceHandler{internalStorage: make(map[string][]byte)}
}

func (h *OccurrenceHandler) Handle(alert alert_entity.AlertEntity) error {
	if h.alertExistsInStorage(&alert) {
		return h.existingAlert(&alert)
	}
	if alert.GetState() == "firing" {
		return h.newOccurrence(false, &alert)
	}
	return nil
}
func (h *OccurrenceHandler) existingAlert(alert *alert_entity.AlertEntity) error {
	oldAlert, err := h.getAlertFromStorage(alert)
	if err != nil {
		return err
	}
	if alert.GetState() == "firing" {
		if oldAlert.GetState() == "firing" {
			return nil
		}
		return h.newOccurrence(true, alert)
	}
	if oldAlert.GetState() == "resolved" {
		return nil
	}
	return h.resolve(alert)
}
func (h *OccurrenceHandler) getAlertFromStorage(alert *alert_entity.AlertEntity) (alert_entity.AlertEntity, error) {
	if h.alertExistsInStorage(alert) {
		oldAlertBytes, ok := h.internalStorage[alert.GetOccurrenceId()]
		if !ok {
			return alert_entity.AlertEntity{}, fmt.Errorf("failed to get alert from internal storage")
		}
		oldAlert, err := alert_entity.NewAlertEntityFromBytes(oldAlertBytes)
		if err != nil {
			return alert_entity.AlertEntity{}, err
		}
		return *oldAlert, nil
	}
	return alert_entity.AlertEntity{}, fmt.Errorf("alert not found in internal storage")
}

func (h *OccurrenceHandler) alertExistsInStorage(alert *alert_entity.AlertEntity) bool {
	_, ok := h.internalStorage[alert.GetAlertId()]
	return ok
}

func (h *OccurrenceHandler) pushToStorage(alert alert_entity.AlertEntity) error {
	alertBytes, err := alert.ToByte()
	if err != nil {
		return err
	}
	log.Println(alert)
	h.internalStorage[alert.GetAlertId()] = alertBytes
	return nil
}

func (h *OccurrenceHandler) newOccurrence(reopen bool, alert *alert_entity.AlertEntity) error {
	if reopen {
		err := alert.SetDescription(alert.GetDescription() + " reopen ")
		if err != nil {
			return err
		}
	}
	err := alert.SetOccurrenceId(strconv.FormatInt(time.Now().UnixNano(), 10))
	if err != nil {
		return err
	}
	return h.pushToStorage(*alert)
}

func (h *OccurrenceHandler) resolve(alert *alert_entity.AlertEntity) error {
	oldAlert, err := h.getAlertFromStorage(alert)
	if err != nil {
		return err
	}
	err = oldAlert.SetState("resolved")
	if err != nil {
		return err
	}
	return h.pushToStorage(oldAlert)
}
