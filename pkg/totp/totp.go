package totp

import (
	"time"

	"github.com/pquerna/otp/totp"
)

// GenerateTOTP generates a 6‑digit TOTP code for the given secret.
// It uses a 30‑second period and SHA‑1 as the algorithm.
func GenerateTOTP(secret string) (string, error) {
	opts := totp.ValidateOpts{
		Period:    30,
		Skew:      1,
		Digits:    totp.DigitsSix,
		Algorithm: totp.AlgorithmSHA1,
	}
	// totp.GenerateCodeCustom automatically decodes the BASE32 secret.
	return totp.GenerateCodeCustom(secret, time.Now(), opts)
}
