package main

import (
	"crypto/rand"
	"crypto/cipher"
	"encoding/hex"
	"fmt"
	"golang.org/x/crypto/chacha20poly1305"
)

const (
	AES256KeyLen  = 32
	AES256NonceLen = 12
)

func encrypt(plaintext string, key []byte, nonce []byte) (string, error) {
	cipher, err := chacha20poly1305.NewX(key)
	if err != nil {
		return "", err
	}
	aesEncrypted := cipher.Seal(nil, nonce, []byte(plaintext), nil)
	hexEncoded := hex.EncodeToString(aesEncrypted)
	return hexEncoded, nil
}

func decrypt(ciphertext string, key []byte, nonce []byte) (string, error) {
	cipher, err := chacha20poly1305.NewX(key)
	if err != nil {
		return "", err
	}
	
	hexDecoded, err := hex.DecodeString(ciphertext)
	if err != nil {
		return "", err
	}

	aesDecrypted, err := cipher.Open(nil, nonce, hexDecoded, nil)
	if err != nil {
		return "", err
	}

	return string(aesDecrypted), nil
}

func generateKey() ([]byte, error) {
	key := make([]byte, AES256KeyLen)
	_, err := rand.Read(key)
	if err != nil {
		return nil, err
	}
	return key, nil
}

func generateNonce() ([]byte, error) {
	nonce := make([]byte, AES256NonceLen)
	_, err := rand.Read(nonce)
	if err != nil {
		return nil, err
	}
	return nonce, nil
}

func main() {
	key, _ := generateKey()
	nonce, _ := generateNonce()
	plaintext := "Hello, World!"

	encrypted, _ := encrypt(plaintext, key, nonce)
	fmt.Println("Encrypted:", encrypted)

	decrypted, _ := decrypt(encrypted, key, nonce)
	fmt.Println("Decrypted:", decrypted)
}