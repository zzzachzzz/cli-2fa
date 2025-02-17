package storage

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"

	"github.com/zzzachzzz/cli-2fa/pkg/secret"
)

// Storage interface for accessing TOTP entries
type Storage struct {
	// Underlying data type stored in encrypted json file
	jsonMap map[string]string
}

func New() Storage {
	return Storage{jsonMap: make(map[string]string)}
}

func (s *Storage) GetTotpSecret(name string) (string, error) {
	if totpSecret, ok := s.jsonMap[name]; !ok {
		return "", fmt.Errorf("No TOTP entry found by name '%s'.", name)
	} else {
		return totpSecret, nil
	}
}

func (s *Storage) SetTotpSecret(name, totpSecret string) {
	s.jsonMap[name] = totpSecret
}

// getStorageFilePath returns the path to the encrypted storage file.
func GetStorageFilePath() (string, error) {
	// Use the OS-specific configuration directory.
	configDir, err := os.UserConfigDir()
	if err != nil {
		return "", err
	}
	dir := filepath.Join(configDir, "cli-2fa")
	os.MkdirAll(dir, os.ModePerm)
	return filepath.Join(dir, "storage.json.enc"), nil
}

// ReadStorage decrypts and parses the storage file using the provided key and nonce.
func ReadStorage(key [secret.AES256KeyLen]byte, nonce [secret.AES256NonceLen]byte) (Storage, error) {
	var jsonMap map[string]string
	var stg Storage
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
	err = json.Unmarshal([]byte(decrypted), &jsonMap)
	if err != nil {
		return stg, err
	}
	stg = Storage{jsonMap}
	return stg, err
}

// WriteStorage encrypts the storage struct in JSON format and writes it to the file.
func WriteStorage(stg Storage, key [secret.AES256KeyLen]byte, nonce [secret.AES256NonceLen]byte) error {
	path, err := GetStorageFilePath()
	if err != nil {
		return err
	}
	jsonData, err := json.Marshal(stg.jsonMap)
	if err != nil {
		return err
	}
	encrypted, err := secret.Encrypt(string(jsonData), key, nonce)
	if err != nil {
		return err
	}
	return ioutil.WriteFile(path, []byte(encrypted), 0644)
}
