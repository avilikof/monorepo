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
	_, err := env_var.NewEnvVarHandler()
	assert.NoError(t, err)
}

// TestLoadDotEnv tests loading environment variables from a .env file.
func TestLoadDotEnv(t *testing.T) {
	// Setup: Create a temporary .env file.
	file, err := os.Create("./.env")
	if err != nil {
		t.Fatalf("Unable to create temporary .env file: %v", err)
	}
	defer os.Remove(file.Name()) // Clean up

	_, err = file.WriteString("TEST_VAR=hello\n")
	if err != nil {
		t.Fatalf("Unable to write to temporary .env file: %v", err)
	}
	file.Close()

	handler, err := env_var.NewEnvVarHandler()
	assert.NoError(t, err)

	err = handler.LoadDotEnv(file.Name())
	assert.NoError(t, err)

	// Test retrieving the variable set in the .env file
	val, err := handler.Get("TEST_VAR")
	assert.NoError(t, err)
	assert.Equal(t, "hello", val)
}

// TestGet tests retrieving environment variables through the handler.
func TestGet(t *testing.T) {
	const testKey = "SOME_VAR"
	const testValue = "some_value"

	// Setup: Set an environment variable
	err := os.Setenv(testKey, testValue)
	if err != nil {
		log.Println(err)
	}
	defer os.Unsetenv(testKey) // Clean up

	handler, err := env_var.NewEnvVarHandler()
	assert.NoError(t, err)

	// Test retrieving the variable
	val, err := handler.Get(testKey)
	assert.NoError(t, err)
	assert.Equal(t, testValue, val)

	// Test retrieving a non-existent variable
	_, err = handler.Get("NON_EXISTENT_VAR")
	assert.Error(t, err)

	// Test retrieving with an empty key
	_, err = handler.Get("")
	assert.Error(t, err)
}
