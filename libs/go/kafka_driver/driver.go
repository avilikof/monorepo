package kafka_driver

import (
	"fmt"
	"github.com/avilikof/monorepo/lib/go/env_var"
	"github.com/confluentinc/confluent-kafka-go/v2/kafka"
)

type KafkaHandler struct {
	consumer *kafka.Consumer
	producer *kafka.Producer
}

func NewKafkaHandler(envVars *env_var.EnvironmentVarHandler) KafkaHandler {
	return KafkaHandler{}
}

func (kh *KafkaHandler) Subscribe(envVars *env_var.EnvironmentVarHandler) error {
	if kh.SubscriptionIsActive() {
		return nil
	}

	configMap, err := setConfigMap(envVars)
	if err != nil {
		return err
	}

	groupId, offset, err := getConsumerEnvVars(envVars)
	if err != nil {
		return err
	}

	cons, err := createKafkaConsumer(groupId, offset, configMap)
	if err != nil {
		return err
	}
	return kh.setConsumer(cons)
}
func (kh *KafkaHandler) SubscriptionIsActive() bool {
	if kh.consumer == nil {
		return false
	}
	return kh.consumer.IsClosed()
}

func (kh *KafkaHandler) setConsumer(cons *kafka.Consumer) error {
	kh.consumer = cons
	return nil
}

func getConsumerEnvVars(envVars *env_var.EnvironmentVarHandler) (string, string, error) {

	groupId, err := envVars.Get("GROUP_ID")
	if err != nil {
		return "", "", err
	}

	offset, err := envVars.Get("KAFKA_OFFSET")
	if err != nil {
		return "", "", err
	}
	return groupId, offset, nil
}

func setConfigMap(envVars *env_var.EnvironmentVarHandler) (*kafka.ConfigMap, error) {
	kafkaBootstrap, err := envVars.Get("KAFKA_URL")
	if err != nil {
		return nil, err
	}
	kafkaUser, err := envVars.Get("KAFKA_USER")
	if err != nil {
		return nil, err
	}
	kafkaPass, err := envVars.Get("KAFKA_PASS")
	if err != nil {
		return nil, err
	}

	configMap := &kafka.ConfigMap{
		"bootstrap.servers":  kafkaBootstrap,
		"security.protocol":  "sasl_ssl",
		"sasl.mechanisms":    "SCRAM-SHA-256",
		"sasl.username":      kafkaUser,
		"sasl.password":      kafkaPass,
		"session.timeout.ms": 60000,
	}
	return configMap, nil
}
func createKafkaConsumer(groupId, offset string, cfg *kafka.ConfigMap) (*kafka.Consumer, error) {
	err := cfg.SetKey("group.id", groupId)
	if err != nil {
		fmt.Printf("failed to add groupId to consumer configuration: %v", err)
		return nil, err
	}
	err = cfg.SetKey("auto.offset.reset", offset)
	if err != nil {
		fmt.Printf("failed to add offset to consumer configuration: %v\n", err)
		return nil, err
	}

	return kafka.NewConsumer(cfg)
}
