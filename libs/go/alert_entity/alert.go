package alert_entity

import (
	"time"
)

type AlertEntity struct {
	fingerprint string
	timestamp   int64
	description string
	state       string
}

func DefaultAlert() *AlertEntity {
	return &AlertEntity{
		fingerprint: "",
		timestamp:   time.Now().UnixNano(),
		description: "random alert",
		state:       "firing",
	}
}
