module monorepo

go 1.21.6

replace (
	github.com/avilikof/monorepo/lib/go/alert_entity => ./libs/go/alert_entity
	github.com/avilikof/monorepo/lib/go/env_var => ./libs/go/env_var
	github.com/avilikof/monorepo/lib/go/kafka_driver => ./libs/go/kafka_driver
	github.com/avilikof/monorepo/tests => ./tests
)
