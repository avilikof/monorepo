package kafkadriver

import (
	"fmt"
	"time"

	"github.com/confluentinc/confluent-kafka-go/v2/kafka"
)

type KafkaHandler struct {
	consumer *kafka.Consumer
	producer *kafka.Producer
	cfgMap   *kafka.ConfigMap
}

func NewKafkaHandler(bootstrapAddress string) *KafkaHandler {
	newConfigMap(bootstrapAddress)
	return &KafkaHandler{
		cfgMap: newConfigMap(bootstrapAddress),
	}
}

func (kh *KafkaHandler) Subscribe(topic string) error {
	if kh.SubscriptionIsActive() {
		return nil
	}

	groupId, offset, _err := getConsumerEnvVars()
	if _err != nil {
		return _err
	}

	cons, _err := createKafkaConsumer(groupId, offset, kh.cfgMap)
	if _err != nil {
		return _err
	}
	_err = cons.Subscribe(topic, nil)
	kh.cfgMap.SetKey("go.application.rebalance.enable", false)
	if _err != nil {
		return _err
	}
	return kh.setConsumer(cons)
}
func (kh *KafkaHandler) SetConfigValues(key string, value kafka.ConfigValue) error {
	return kh.cfgMap.SetKey(key, value)
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
	message, err := kh.consumer.ReadMessage(10 * time.Second)
	if err != nil {
		return kafka.Message{}, err
	}
	return *message, nil
}
func (kh *KafkaHandler) Push(key, value []byte, topic string) error {
	if kh.cfgMap == nil {
		panic("No configmap")
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

func getConsumerEnvVars() (string, string, error) {
	return "Group_1", "latest", nil
}
func newConfigMap(bootstrapAddress string) *kafka.ConfigMap {

	configMap := &kafka.ConfigMap{
		"bootstrap.servers": bootstrapAddress,
	}
	return configMap
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
	fmt.Println(cfg)
	return kafka.NewProducer(cfg)
}
func createMessage(key, value []byte, topic string) kafka.Message {
	return kafka.Message{TopicPartition: kafka.TopicPartition{Topic: &topic}, Key: key, Value: value}
}
