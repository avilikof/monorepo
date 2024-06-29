module github.com/avilikof/monorepo/apps/go/nats-client

go 1.22.2

require (
	github.com/avilikof/monorepo/libs/go/alert_entity v0.1.0
	github.com/avilikof/monorepo/libs/go/nats_driver v0.1.0
)

require (
	github.com/klauspost/compress v1.17.8 // indirect
	github.com/nats-io/nats.go v1.35.0 // indirect
	github.com/nats-io/nkeys v0.4.7 // indirect
	github.com/nats-io/nuid v1.0.1 // indirect
	golang.org/x/crypto v0.23.0 // indirect
	golang.org/x/sys v0.20.0 // indirect
)

replace github.com/avilikof/monorepo/libs/go/alert_entity => ../../../libs/go/alert_entity

replace github.com/avilikof/monorepo/libs/go/nats_driver => ../../../libs/go/nats_driver
