package alerthandler

import "go-mono/pkg/alerts/alert_entity"

type AlertHandler struct{}

type storageInterface interface {
	Pull(string) ([]byte, error)
	Push(string, []byte) error
}

func NewAlertHandler(pubSub storageInterface) *AlertHandler {
	return &AlertHandler{}
}

func (self *AlertHandler) PullNewAlert(subject string, storage storageInterface) (*alert_entity.AlertEntity, error) {
	alertBytes, err := storage.Pull(subject)
	if err != nil {
		return nil, err
	}
	alert, err := alert_entity.NewAlertEntityFromBytes(alertBytes)
	if err != nil {
		return nil, err
	}
	return alert, nil
}
