package cmd

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/zzzachzzz/cli-2fa/pkg/keyring"
	"github.com/zzzachzzz/cli-2fa/pkg/secret"
	"github.com/zzzachzzz/cli-2fa/pkg/storage"

	"github.com/spf13/cobra"
)

var addCmd = &cobra.Command{
	Use:   "add",
	Short: "Add a new 2FA entry",
	RunE: func(cmd *cobra.Command, args []string) error {
		reader := bufio.NewReader(os.Stdin)
		fmt.Print("Enter name for 2FA entry: ")
		name, _ := reader.ReadString('\n')
		name = strings.TrimSpace(name)
		fmt.Print("Enter secret for 2FA entry: ")
		sec, _ := reader.ReadString('\n')
		sec = strings.TrimSpace(sec)

		// Retrieve the encryption key and nonce from the keyring.
		key, nonce, err := keyring.GetKeyringEntry()
		if err != nil {
			// If the keyring entry doesn't exist, generate and store new ones.
			key = secret.GenerateKey()
			nonce = secret.GenerateNonce()
			err = keyring.SetKeyringEntry(key, nonce)
			if err != nil {
				return err
			}
		}

		// Read existing storage.
		stg, err := storage.ReadStorage(key, nonce)
		if err != nil {
			// If the file does not exist, start with an empty store.
			stg = storage.New()
		}

		// Insert the new entry.
		// TODO Consider warning on overwrite of entry with same name
		stg.SetTotpSecret(name, sec)

		// Generate a new nonce for the updated file.
		newNonce := secret.GenerateNonce()

		// Write the updated storage.
		if err := storage.WriteStorage(stg, key, newNonce); err != nil {
			return err
		}

		// Update the keyring entry with the new nonce.
		if err := keyring.SetKeyringEntry(key, newNonce); err != nil {
			return err
		}

		fmt.Printf("Added 2FA entry \"%s\"\n", name)
		return nil
	},
}

func init() {
	rootCmd.AddCommand(addCmd)
}
