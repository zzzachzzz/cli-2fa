package keyring

import (
	"encoding/hex"
	"errors"
	"fmt"

	"github.com/zalando/go-keyring"

	"github.com/zzzachzzz/cli-2fa/pkg/secret"
)

// service and user are constants to define the keyring entry.
const (
	service = "cli-2fa"
	user    = "totp-secret-encryption-key"
)

// GetKeyringEntry reads the hex‐encoded combined key (32 bytes) and nonce (12 bytes)
// from the keyring.
func GetKeyringEntry() (key [secret.AES256KeyLen]byte, nonce [secret.AES256NonceLen]byte, err error) {
	entry, err := keyring.Get(service, user)
	if err != nil {
		return key, nonce, err
	}
	combined, err := hex.DecodeString(entry)
	if err != nil {
		return key, nonce, err
	}
	if len(combined) != secret.AES256KeyLen+secret.AES256NonceLen {
		err := errors.New(fmt.Sprintf(
			"The retrieved keyring value did not match the expected length for key and nonce. Got length: %d",
			len(combined),
		))
		return key, nonce, err
	}

	// Convert slices of `combined` to fixed length byte arrays
	key = [secret.AES256KeyLen]byte(combined[:secret.AES256KeyLen])
	nonce = [secret.AES256NonceLen]byte(combined[secret.AES256KeyLen:])
	return key, nonce, nil
}

// SetKeyringEntry combines the key and nonce, hex‑encodes them,
// and saves them to the keyring.
func SetKeyringEntry(key [secret.AES256KeyLen]byte, nonce [secret.AES256NonceLen]byte) error {
	combined := append(key[:], nonce[:]...)
	entry := hex.EncodeToString(combined)
	return keyring.Set(service, user, entry)
}
