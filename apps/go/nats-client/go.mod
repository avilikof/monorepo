module github.com/avilikof/monorepo/apps/go/nats-client

go 1.22.0

require (
	github.com/avilikof/monorepo/libs/go/alert_entity v0.1.0
	github.com/avilikof/monorepo/libs/go/nats_driver v0.1.0
	golang.org/x/sys v0.0.0
)

replace github.com/avilikof/monorepo/libs/go/alert_entity => ../../../libs/go/alert_entity

replace github.com/avilikof/monorepo/libs/go/nats_driver => ../../../libs/go/nats_driver
