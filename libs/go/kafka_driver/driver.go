package kafka_driver

import (
	"fmt"
	"time"

	"github.com/avilikof/monorepo/lib/go/env_var"
	"github.com/confluentinc/confluent-kafka-go/v2/kafka"
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

	configMap, _err := newConfigMap(envVars)
	if _err != nil {
		return _err
	}

	groupId, offset, _err := getConsumerEnvVars(envVars)
	if _err != nil {
		return _err
	}

	cons, _err := createKafkaConsumer(groupId, offset, configMap)
	if _err != nil {
		return _err
	}
	topic, _err := envVars.Get("KAFKA_TOPIC")
	if _err != nil {
		return _err
	}
	_err = cons.Subscribe(topic, nil)
	if _err != nil {
		return _err
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
	message, _err := kh.consumer.ReadMessage(120 * time.Second)
	if _err != nil {
		return kafka.Message{}, _err
	}
	return *message, nil
}
func (kh *KafkaHandler) Push(key, value []byte, topic string) error {

	if kh.cfgMap == nil {
		cfgMap, _err := newConfigMap(kh.envVars)
		if _err != nil {
			return _err
		}
		_err = kh.setConfigMap(cfgMap)
		if _err != nil {
			return _err
		}
	}
	if kh.producer == nil || !kh.ProducerIsAlive() {
		producer, _err := createKafkaProducer(kh.cfgMap)
		if _err != nil {
			return _err
		}
		_err = kh.setProducer(producer)
		if _err != nil {
			return _err
		}
	}
	kafkaMessage := createMessage(key, value, topic)
	return kh.producer.Produce(&kafkaMessage, nil)
}

func getConsumerEnvVars(envVars *env_var.EnvironmentVarHandler) (string, string, error) {

	groupId, _err := envVars.Get("GROUP_ID")
	if _err != nil {
		return "", "", _err
	}

	offset, _err := envVars.Get("KAFKA_OFFSET")
	if _err != nil {
		return "", "", _err
	}
	return groupId, offset, nil
}
func newConfigMap(envVars *env_var.EnvironmentVarHandler) (*kafka.ConfigMap, error) {
	kafkaBootstrap, _err := envVars.Get("KAFKA_URL")
	if _err != nil {
		return nil, _err
	}
	kafkaUser, _err := envVars.Get("KAFKA_USER")
	if _err != nil {
		return nil, _err
	}
	kafkaPass, _err := envVars.Get("KAFKA_PASS")
	if _err != nil {
		return nil, _err
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
	_err := cfg.SetKey("group.id", groupId)
	if _err != nil {
		fmt.Printf("failed to add groupId to consumer configuration: %v", _err)
		return nil, _err
	}
	_err = cfg.SetKey("auto.offset.reset", offset)
	if _err != nil {
		fmt.Printf("failed to add offset to consumer configuration: %v\n", _err)
		return nil, _err
	}
	_err = cfg.SetKey("session.timeout.ms", 60000)
	if _err != nil {
		fmt.Printf("failed to add session.timeout.ms to consumer configuration: %v\n", _err)
		return nil, _err
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
