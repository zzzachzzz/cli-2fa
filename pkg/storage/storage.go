package storage

import (
	"encoding/json"
	"io/ioutil"
	"log"
	"os"
	"path/filepath"

	"github.com/zzzachzzz/cli-2fa/pkg/secret"
)

// Storage represents a map of TOTP entries.
type Store struct {
	Map map[string]string `json:"map"`
}

// getStorageFilePath returns the path to the encrypted storage file.
func GetStorageFilePath() (string, error) {
	// Use the OS-specific configuration directory.
	configDir, err := os.UserConfigDir()
	if err != nil {
		return "", err
	}

	log.Println("configDir: ", configDir)
	dir := filepath.Join(configDir, "cli-2fa")
	os.MkdirAll(dir, os.ModePerm)
	return filepath.Join(dir, "storage.json.enc"), nil
}

// ReadStorage decrypts and parses the storage file using the provided key and nonce.
func ReadStorage(key, nonce []byte) (Store, error) {
	var stg Store
	path, err := GetStorageFilePath()
	if err != nil {
		return stg, err
	}
	data, err := ioutil.ReadFile(path)
	if err != nil {
		return stg, err
	}
	// Decrypt the file contents.
	decrypted, err := secret.Decrypt(string(data), key, nonce)
	if err != nil {
		return stg, err
	}
	err = json.Unmarshal([]byte(decrypted), &stg)
	return stg, err
}

// WriteStorage encrypts the storage struct in JSON format and writes it to the file.
func WriteStorage(stg Store, key, nonce []byte) error {
	path, err := GetStorageFilePath()
	if err != nil {
		return err
	}
	jsonData, err := json.Marshal(stg)
	if err != nil {
		return err
	}
	encrypted, err := secret.Encrypt(string(jsonData), key, nonce)
	if err != nil {
		return err
	}
	return ioutil.WriteFile(path, []byte(encrypted), 0644)
}
