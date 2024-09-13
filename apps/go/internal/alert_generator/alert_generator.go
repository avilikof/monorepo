package alertgenerator

import (
	"go-mono/pkg/alerts/alert_entity"
)

func newRandomAlert() *alert_entity.AlertEntity {
	alert := alert_entity.RandomAlert(10000)
	return &alert
}

func PublishAlert(subbject string, pubInt PubInterface) error {
	alert := newRandomAlert()
	alertAsByte, err := alert.ToByte()
	if err != nil {
		return err
	}
	pubInt.Publish(subbject, alertAsByte)
	return nil
}
