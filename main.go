package main

import (
	"fmt"
	"os"

	"cli-2fa/cmd"
)

func main() {

	if err := cmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
