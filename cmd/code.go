/*
Copyright © 2025 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"

	"github.com/zzzachzzz/cli-2fa/pkg/keyring"
	"github.com/zzzachzzz/cli-2fa/pkg/storage"
	"github.com/zzzachzzz/cli-2fa/pkg/totp"
)

// codeCmd represents the code command
var codeCmd = &cobra.Command{
	Use:   "code [entry]",
	Args:  cobra.ExactArgs(1),
	Short: "Get a 2FA code for a specific entry",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		entryArg := args[0] // 1 arg ensured with `cobra.ExactArgs(1)`
		key, nonce, err := keyring.GetKeyringEntry()
		if err != nil {
			return err
		}
		stg, err := storage.ReadStorage(key, nonce)
		if err != nil {
			return err
		}
		totpSecret, err := stg.GetTotpSecret(entryArg)
		if err != nil {
			return err
		}
		totp, err := totp.GenerateTOTP(totpSecret)
		if err != nil {
			return err
		}
		fmt.Println(totp)
		return nil
	},
}

func init() {
	rootCmd.AddCommand(codeCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// codeCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// codeCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
