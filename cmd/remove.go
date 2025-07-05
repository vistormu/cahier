package cmd

import (
	"github.com/spf13/cobra"
)

var removeCmd = &cobra.Command{
	Use:   "remove",
	Short: "remove an SSH connection",
	Long:  `remove an existing SSH connection by its nickname from the Cahier page manager.`,
	Args:  cobra.ExactArgs(1),
	RunE:  deleteCmd.RunE,
}

func init() {
	rootCmd.AddCommand(removeCmd)
}
