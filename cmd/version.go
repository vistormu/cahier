package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
	"github.com/vistormu/go-dsa/ansi"
)

var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "print the version number of Cahier",
	Long:  `all software has versions. This is Cahier's version.`,
	Args:  cobra.NoArgs,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Printf("%s version %s\n",
			cahierName,
			ansi.Cyan+Version+ansi.Reset,
		)
	},
}

func init() {
	rootCmd.AddCommand(versionCmd)
}
