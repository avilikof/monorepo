module github.com/avilikof/monorepo/apps/go/productivity_tool/occurrence_tracker

go 1.22.0

replace (
	github.com/avilikof/monorepo/lib/go/alert_entity => ../../../../libs/go/alert_entity
	github.com/avilikof/monorepo/lib/go/env_var => ../../../../libs/go/env_var
	github.com/avilikof/monorepo/lib/go/kafka_driver => ../../../../libs/go/kafka_driver
)
