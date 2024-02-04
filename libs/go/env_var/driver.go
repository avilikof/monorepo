package env_var

import (
	"fmt"
	"github.com/spf13/viper"
)

type EnvironmentVarHandler struct{}

func NewEnvVarHandler(prefix *string) (EnvironmentVarHandler, error) {
	err := loadSystemVariables(prefix)
	return EnvironmentVarHandler{}, err
}

func loadSystemVariables(prefix *string) error {
	if len(*prefix) != 0 {
		viper.SetEnvPrefix(*prefix)
	}
	viper.AutomaticEnv()
	return nil
}

func (e *EnvironmentVarHandler) LoadDotEnv(path, prefix *string) error {
	viper.AddConfigPath(*path)
	viper.SetConfigFile(".env")
	if err := viper.ReadInConfig(); err != nil {
		return err
	}
	return nil
}

func (e *EnvironmentVarHandler) Get(key *string) (string, error) {
	if len(*key) == 0 {
		return "", fmt.Errorf("key cannot be empty string")
	}
	value := viper.GetString(*key)
	if len(value) != 0 {
		return value, nil
	}
	return value, fmt.Errorf("key not found: %s", *key)
}
