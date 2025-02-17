package main

import (
	"fmt"
	"os"

	"github.com/zzzachzzz/cli-2fa/cmd"
)

func main() {
	// Might be a good error handling / logging solution:
	// https://github.com/spf13/cobra/issues/914#issuecomment-548411337
	if err := cmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
