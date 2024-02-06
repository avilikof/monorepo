package kafka_driver

import (
	"fmt"
	"github.com/avilikof/monorepo/lib/go/env_var"
	"github.com/confluentinc/confluent-kafka-go/v2/kafka"
	"time"
)

type KafkaHandler struct {
	consumer *kafka.Consumer
	producer *kafka.Producer
	cfgMap   *kafka.ConfigMap
	envVars  *env_var.EnvironmentVarHandler
}

func NewKafkaHandler(envVars *env_var.EnvironmentVarHandler) KafkaHandler {
	return KafkaHandler{envVars: envVars}
}

func (kh *KafkaHandler) Subscribe(envVars *env_var.EnvironmentVarHandler) error {
	if kh.SubscriptionIsActive() {
		return nil
	}

	configMap, err := newConfigMap(envVars)
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
	topic, err := envVars.Get("KAFKA_TOPIC")
	if err != nil {
		return err
	}
	err = cons.Subscribe(topic, nil)
	if err != nil {
		return err
	}
	return kh.setConsumer(cons)
}
func (kh *KafkaHandler) SubscriptionIsActive() bool {
	if kh.consumer != nil {
		return !kh.consumer.IsClosed()
	}
	return false
}
func (kh *KafkaHandler) ProducerIsAlive() bool {
	if kh.producer != nil {
		return !kh.producer.IsClosed()
	}
	return false
}
func (kh *KafkaHandler) setConsumer(cons *kafka.Consumer) error {
	kh.consumer = cons
	return nil
}
func (kh *KafkaHandler) setConfigMap(cfgMap *kafka.ConfigMap) error {
	kh.cfgMap = cfgMap
	return nil
}
func (kh *KafkaHandler) setProducer(producer *kafka.Producer) error {
	kh.producer = producer
	return nil
}
func (kh *KafkaHandler) Get() (kafka.Message, error) {
	message, err := kh.consumer.ReadMessage(120 * time.Second)
	if err != nil {
		return kafka.Message{}, err
	}
	return *message, nil
}
func (kh *KafkaHandler) Push(key, value []byte, topic string) error {

	if kh.cfgMap == nil {
		cfgMap, err := newConfigMap(kh.envVars)
		if err != nil {
			return err
		}
		err = kh.setConfigMap(cfgMap)
		if err != nil {
			return err
		}
	}
	if kh.producer == nil || !kh.ProducerIsAlive() {
		producer, err := createKafkaProducer(kh.cfgMap)
		if err != nil {
			return err
		}
		err = kh.setProducer(producer)
		if err != nil {
			return err
		}
	}
	kafkaMessage := createMessage(key, value, topic)
	return kh.producer.Produce(&kafkaMessage, nil)
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
func newConfigMap(envVars *env_var.EnvironmentVarHandler) (*kafka.ConfigMap, error) {
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
		"bootstrap.servers": kafkaBootstrap,
		"security.protocol": "sasl_ssl",
		"sasl.mechanisms":   "SCRAM-SHA-256",
		"sasl.username":     kafkaUser,
		"sasl.password":     kafkaPass,
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
	err = cfg.SetKey("session.timeout.ms", 60000)
	if err != nil {
		fmt.Printf("failed to add session.timeout.ms to consumer configuration: %v\n", err)
		return nil, err
	}

	return kafka.NewConsumer(cfg)
}
func createKafkaProducer(cfg *kafka.ConfigMap) (*kafka.Producer, error) {
	if cfg == nil {
		return nil, fmt.Errorf("config map not set")
	}
	return kafka.NewProducer(cfg)
}
func createMessage(key, value []byte, topic string) kafka.Message {
	return kafka.Message{TopicPartition: kafka.TopicPartition{Topic: &topic}, Key: key, Value: value}
}
