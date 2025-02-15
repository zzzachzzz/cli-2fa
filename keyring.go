package main

import (
	"encoding/hex"
	"errors"
	"fmt"
	"github.com/zalando/go-keyring"
)

const (
	KeyringService = "cli-2fa"
	KeyringUser    = "totp-secret-encryption-key"
	AES256KeyLen   = 32
	AES256NonceLen = 12
)

func getKeyringEntryKeyAndNonce() ([]byte, []byte, error) {
	keyringEntryPassword, err := keyring.Get(KeyringService, KeyringUser)
	if err != nil {
		return nil, nil, err
	}

	bytes, err := hex.DecodeString(keyringEntryPassword)
	if err != nil {
		return nil, nil, errors.New("unable to hex decode password from keyring")
	}

	if len(bytes) != AES256KeyLen+AES256NonceLen {
		return nil, nil, fmt.Errorf("invalid key length: expected %d, got %d", AES256KeyLen+AES256NonceLen, len(bytes))
	}

	key := bytes[:AES256KeyLen]
	nonce := bytes[AES256KeyLen:]

	return key, nonce, nil
}

func setKeyringEntry(aesKey, aesNonce []byte) error {
	if len(aesKey) != AES256KeyLen || len(aesNonce) != AES256NonceLen {
		return errors.New("invalid key or nonce length")
	}

	combined := append(aesKey, aesNonce...)
	keyringEntryPassword := hex.EncodeToString(combined)

	return keyring.Set(KeyringService, KeyringUser, keyringEntryPassword)
}