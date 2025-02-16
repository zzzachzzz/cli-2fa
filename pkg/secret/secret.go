package secret

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/hex"
	"io"
)

const (
	AES256KeyLen   = 32
	AES256NonceLen = 12
)

// Encrypt encrypts the plaintext string using AES‑GCM with the provided key and nonce,
// and returns a hex‑encoded ciphertext.
func Encrypt(plaintext string, key [AES256KeyLen]byte, nonce [AES256NonceLen]byte) (string, error) {
	block, err := aes.NewCipher(key[:])
	if err != nil {
		return "", err
	}
	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		return "", err
	}
	ciphertext := aesgcm.Seal(nil, nonce[:], []byte(plaintext), nil)
	return hex.EncodeToString(ciphertext), nil
}

// Decrypt decodes the hex‑encoded ciphertext and decrypts it using AES‑GCM
// with the given key and nonce.
func Decrypt(ciphertextHex string, key [AES256KeyLen]byte, nonce [AES256NonceLen]byte) (string, error) {
	ciphertext, err := hex.DecodeString(ciphertextHex)
	if err != nil {
		return "", err
	}
	block, err := aes.NewCipher(key[:])
	if err != nil {
		return "", err
	}
	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		return "", err
	}
	plaintext, err := aesgcm.Open(nil, nonce[:], ciphertext, nil)
	if err != nil {
		return "", err
	}
	return string(plaintext), nil
}

// GenerateKey creates a 32‑byte encryption key.
func GenerateKey() [AES256KeyLen]byte {
	key := make([]byte, AES256KeyLen)
	_, _ = io.ReadFull(rand.Reader, key)
	return [AES256KeyLen]byte(key)
}

// GenerateNonce creates a 12‑byte nonce.
func GenerateNonce() [AES256NonceLen]byte {
	nonce := make([]byte, AES256NonceLen)
	_, _ = io.ReadFull(rand.Reader, nonce)
	return [AES256NonceLen]byte(nonce)
}
