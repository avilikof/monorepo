module monorepo

go 1.21.6

replace (
	github.com/avilikof/monorepo/lib/go/alert_entity => ./libs/go/alert_entity
	github.com/avilikof/monorepo/lib/go/env_var => ./libs/go/env_var
	github.com/avilikof/monorepo/lib/go/kafka_driver => ./libs/go/kafka_driver
	github.com/avilikof/monorepo/tests => ./tests
)

require (
	github.com/klauspost/compress v1.17.2 // indirect
	github.com/nats-io/nats.go v1.33.1 // indirect
	github.com/nats-io/nkeys v0.4.7 // indirect
	github.com/nats-io/nuid v1.0.1 // indirect
	golang.org/x/crypto v0.18.0 // indirect
	golang.org/x/sys v0.16.0 // indirect
)
