package cmd

import (
	"os"

	"github.com/spf13/cobra"
	"github.com/vistormu/go-dsa/ansi"
)

const (
	Version    = "0.2.0"
	cahierName = ansi.Bold + ansi.Italic + ansi.Magenta + "cahier" + ansi.Reset
)

var rootCmd = &cobra.Command{
	Use:   cahierName,
	Short: "a better way to visualize your file system",
	Long:  cahierName + ` is the replacement of ls, tree, and cat commands with a more user-friendly output, with a focus on git repositories`,
}

func init() {
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}
