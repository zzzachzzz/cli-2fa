package keyring

import (
	"encoding/hex"
	"errors"
	"fmt"

	"github.com/zalando/go-keyring"
)

// service and user are constants to define the keyring entry.
const (
	service = "cli-2fa"
	user    = "totp-secret-encryption-key"
)

// GetKeyringEntry reads the hex‐encoded combined key (32 bytes) and nonce (12 bytes)
// from the keyring.
func GetKeyringEntry() (key []byte, nonce []byte, err error) {
	entry, err := keyring.Get(service, user)
	if err != nil {
		return nil, nil, err
	}
	combined, err := hex.DecodeString(entry)
	if err != nil {
		return nil, nil, err
	}
	if len(combined) != 32+12 {
		return nil, nil, errors.New(fmt.Sprintf("invalid combined length: %d", len(combined)))
	}
	key = combined[:32]
	nonce = combined[32:]
	return key, nonce, nil
}

// SetKeyringEntry combines the key and nonce, hex‑encodes them,
// and saves them to the keyring.
func SetKeyringEntry(key, nonce []byte) error {
	combined := append(key, nonce...)
	entry := hex.EncodeToString(combined)
	return keyring.Set(service, user, entry)
}
