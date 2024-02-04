package env_var

import (
	"fmt"
	"github.com/spf13/viper"
	"path/filepath"
)

type EnvironmentVarHandler struct{}

func NewEnvVarHandler() (EnvironmentVarHandler, error) {
	err := loadSystemVariables()
	return EnvironmentVarHandler{}, err
}

func loadSystemVariables() error {
	viper.AutomaticEnv()
	return nil
}

func (e *EnvironmentVarHandler) LoadDotEnv(fullPath string) error {
	dir, file := filepath.Split(fullPath)
	viper.AddConfigPath(dir)
	viper.SetConfigFile(file)
	if err := viper.ReadInConfig(); err != nil {
		return err
	}
	return nil
}

func (e *EnvironmentVarHandler) Get(key string) (string, error) {
	if len(key) == 0 {
		return "", fmt.Errorf("key cannot be empty string")
	}
	value := viper.GetString(key)
	if len(value) != 0 {
		return value, nil
	}
	return value, fmt.Errorf("key not found: %s", key)
}
