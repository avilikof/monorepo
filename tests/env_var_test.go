package tests

import (
	"errors"
	_ "os"
	"testing"

	"github.com/avilikof/monorepo/lib/go/env_var"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

// MockViper for controlled testing
type MockViper struct {
	mock.Mock
}

func (m *MockViper) SetEnvPrefix(prefix string) {
	m.Called(prefix)
}

func (m *MockViper) AutomaticEnv() {
	m.Called()
}

func (m *MockViper) AddConfigPath(path string) {
	m.Called(path)
}

func (m *MockViper) SetConfigFile(filename string) {
	m.Called(filename)
}

func (m *MockViper) ReadInConfig() error {
	return m.Called().Error(0)
}

func (m *MockViper) GetString(key string) string {
	args := m.Called(key)
	return args.String(0)
}

// Test suite
func TestEnvironmentVarHandler(t *testing.T) {
	var mockViper = new(MockViper) // Use a mock Viper for testing

	// Test NewEnvVarHandler
	t.Run("NewEnvVarHandler - success", func(t *testing.T) {
		prefix := "TEST_PREFIX"
		handler, err := env_var.NewEnvVarHandler(&prefix)
		assert.NoError(t, err)
		assert.NotNil(t, handler)
	})

	t.Run("NewEnvVarHandler - error handling", func(t *testing.T) {
		var errPrefix *string // Empty prefix
		handler, err := env_var.NewEnvVarHandler(errPrefix)
		assert.Error(t, err)
		assert.Nil(t, handler)
	})

	// Test loadSystemVariables (using mock Viper)
	t.Run("loadSystemVariables - sets prefix", func(t *testing.T) {
		prefix := "TEST_PREFIX"
		mockViper.AssertCalled(t, "SetEnvPrefix", prefix)
	})

	// Test LoadDotEnv (using mock Viper)
	t.Run("LoadDotEnv - success", func(t *testing.T) {
		path := "test/path"
		prefix := "TEST_PREFIX"
		handler, _ := env_var.NewEnvVarHandler(&prefix)
		mockViper.On("ReadInConfig").Return(nil)
		assert.NoError(t, handler.LoadDotEnv(&path, &prefix))
		mockViper.AssertCalled(t, "AddConfigPath", path)
		mockViper.AssertCalled(t, "SetConfigFile", ".env")
		mockViper.AssertCalled(t, "ReadInConfig")
	})

	t.Run("LoadDotEnv - error handling", func(t *testing.T) {
		path := "invalid/path"
		prefix := "TEST_PREFIX"
		handler, _ := env_var.NewEnvVarHandler(&prefix)
		mockViper.On("ReadInConfig").Return(errors.New("error reading config"))
		assert.Error(t, handler.LoadDotEnv(&path, &prefix))
		mockViper.AssertCalled(t, "ReadInConfig")
	})

	// Test Get (using mock Viper)
	t.Run("Get - valid key", func(t *testing.T) {
		key := "TEST_KEY"
		value := "test_value"
		mockViper.On("GetString", key).Return(value)
		prefix := ""
		handler, _ := env_var.NewEnvVarHandler(&prefix)
		result, err := handler.Get(&key)
		assert.NoError(t, err)
		assert.Equal(t, value, result)
	})

	t.Run("Get - empty key", func(t *testing.T) {
		var emptyKey *string
		prefix := ""
		handler, _ := env_var.NewEnvVarHandler(&prefix)
		result, err := handler.Get(emptyKey)
		assert.Error(t, err)
		assert.Empty(t, result)
	})
}
