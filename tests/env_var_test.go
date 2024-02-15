package tests

import (
	"log"
	"os"
	"testing"

	"github.com/avilikof/monorepo/lib/go/env_var"
	"github.com/stretchr/testify/assert"
)

// TestNewEnvVarHandler tests the initialization of the EnvironmentVarHandler.
func TestNewEnvVarHandler(t *testing.T) {
	_, _err := env_var.NewEnvVarHandler()
	assert.NoError(t, _err)
}

// TestLoadDotEnv tests loading environment variables from a .env file.
func TestLoadDotEnv(t *testing.T) {
	// Setup: Create a temporary .env file.
	file, _err := os.Create("./.env")
	if _err != nil {
		t.Fatalf("Unable to create temporary .env file: %v", _err)
	}
	defer os.Remove(file.Name()) // Clean up

	_, _err = file.WriteString("TEST_VAR=hello\n")
	if _err != nil {
		t.Fatalf("Unable to write to temporary .env file: %v", _err)
	}
	file.Close()

	handler, _err := env_var.NewEnvVarHandler()
	assert.NoError(t, _err)

	_err = handler.LoadDotEnv(file.Name())
	assert.NoError(t, _err)

	// Test retrieving the variable set in the .env file
	val, _err := handler.Get("TEST_VAR")
	assert.NoError(t, _err)
	assert.Equal(t, "hello", val)
}

// TestGet tests retrieving environment variables through the handler.
func TestGet(t *testing.T) {
	const testKey = "SOME_VAR"
	const testValue = "some_value"

	// Setup: Set an environment variable
	_err := os.Setenv(testKey, testValue)
	if _err != nil {
		log.Println(_err)
	}
	defer os.Unsetenv(testKey) // Clean up

	handler, _err := env_var.NewEnvVarHandler()
	assert.NoError(t, _err)

	// Test retrieving the variable
	val, _err := handler.Get(testKey)
	assert.NoError(t, _err)
	assert.Equal(t, testValue, val)

	// Test retrieving a non-existent variable
	_, _err = handler.Get("NON_EXISTENT_VAR")
	assert.Error(t, _err)

	// Test retrieving with an empty key
	_, _err = handler.Get("")
	assert.Error(t, _err)
}
