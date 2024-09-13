package alertgenerator

type PubInterface interface {
	Publish(subject string, payload []byte) error
}
