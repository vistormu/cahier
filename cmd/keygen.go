package cmd

import (
	"cahier/internal/commandmanager"

	"github.com/spf13/cobra"
)

var keyGenCmd = &cobra.Command{
	Use:   "keygen",
	Short: "generate SSH keys",
	Long:  `generate SSH keys for secure authentication with remote servers. This command will create a new SSH key pair if it does not already exist.`,
	Args:  cobra.NoArgs,
	RunE: func(cmd *cobra.Command, args []string) error {
		return commandmanager.KeyGen()
	},
}

func init() {
	rootCmd.AddCommand(keyGenCmd)
}
