package main

import (
	"github.com/avilikof/monorepo/lib/go/alert_entity"
	"log"
)

type OccurrenceHandler struct {
	internalStorage map[string][]byte
}

func NewOccurrenceHandler() OccurrenceHandler {
	return OccurrenceHandler{internalStorage: make(map[string][]byte)}
}

func (h *OccurrenceHandler) handle(alert *alert_entity.AlertEntity) error {
	if h.alertExistsInStorage(alert) {
		log.Println("alert exists")
	}
	log.Println("writing alert to storage")
	return h.pushToStorage(alert)
}

func (h *OccurrenceHandler) alertExistsInStorage(alert *alert_entity.AlertEntity) bool {
	_, ok := h.internalStorage[alert.GetAlertId()]
	return ok
}

func (h *OccurrenceHandler) pushToStorage(alert *alert_entity.AlertEntity) error {
	alertBytes, err := alert.ToByte()
	if err != nil {
		return err
	}
	h.internalStorage[alert.GetAlertId()] = alertBytes
	return nil
}
