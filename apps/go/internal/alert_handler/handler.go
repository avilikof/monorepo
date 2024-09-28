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
	alertBytes, _err := storage.Pull(subject)
	if _err != nil {
		return nil, _err
	}
	alert, _err := alert_entity.NewAlertEntityFromBytes(alertBytes)
	if _err != nil {
		return nil, _err
	}
	return alert, nil
}
