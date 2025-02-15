package main

import (
	"crypto/rand"
	"encoding/json"
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"github.com/adrg/xdg"
)

const (
	AppName          = "cli-2fa"
	StorageFilename  = "storage.json.enc"
	StorageFilenameTmp = "storage.json.enc.tmp"
)

type TotpEntry struct {
	Name   string `json:"name"`
	Secret string `json:"secret"`
}

type Storage struct {
	Map map[string]string `json:"map"`
}

func readFromFile(key, nonce []byte) (*Storage, error) {
	filePath := getStorageFilePath()
	fileContentsEnc, err := ioutil.ReadFile(filePath)
	if err != nil {
		return nil, err
	}
	
	fileContentsPlain, err := decrypt(fileContentsEnc, key, nonce)
	if err != nil {
		return nil, errors.New("unable to decrypt storage file with provided key and nonce")
	}
	
	var storage Storage
	err = json.Unmarshal(fileContentsPlain, &storage)
	if err != nil {
		return nil, err
	}
	
	return &storage, nil
}

func writeToFile(storage *Storage, key, nonce []byte) error {
	filePath := getStorageFilePath()
	return _writeToFile(filePath, storage, key, nonce)
}

func writeToTmpFile(storage *Storage, key, nonce []byte) error {
	filePath := getStorageFilePathTmp()
	return _writeToFile(filePath, storage, key, nonce)
}

func overwriteMainFileWithTmpFile() error {
	mainFilePath := getStorageFilePath()
	tmpFilePath := getStorageFilePathTmp()
	return os.Rename(tmpFilePath, mainFilePath)
}

func _writeToFile(filePath string, storage *Storage, key, nonce []byte) error {
	storageJSON, err := json.Marshal(storage)
	if err != nil {
		return err
	}
	
	storageJSONEnc, err := encrypt(storageJSON, key, nonce)
	if err != nil {
		return err
	}
	
	dir := filepath.Dir(filePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	
	return ioutil.WriteFile(filePath, storageJSONEnc, 0644)
}

func getStorageFilePath() string {
	return filepath.Join(xdg.DataHome, AppName, StorageFilename)
}

func getStorageFilePathTmp() string {
	return filepath.Join(xdg.DataHome, AppName, StorageFilenameTmp)
}

func storageFileExists() bool {
	filePath := getStorageFilePath()
	_, err := os.Stat(filePath)
	return !os.IsNotExist(err)
}

func encrypt(data, key, nonce []byte) ([]byte, error) {
	// Placeholder for actual encryption function
	return data, nil
}

func decrypt(data, key, nonce []byte) ([]byte, error) {
	// Placeholder for actual decryption function
	return data, nil
}