package cmd

import (
	"cahier/internal/pagemanager"
	"cahier/internal/prompter"
	"fmt"

	"github.com/spf13/cobra"
)

var deleteCmd = &cobra.Command{
	Use:   "delete",
	Short: "delete an SSH connection",
	Long:  `delete an existing SSH connection by its nickname from the Cahier page manager.`,
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		nickname := args[0]
		if !pagemanager.SSHConnectionExists(nickname) {
			return fmt.Errorf("SSH connection with nickname '%s' does not exist", nickname)
		}

		if prompter.AskForConfirmDelete(nickname) {
			err := pagemanager.RemoveSSHConnectionFromFile(nickname)
			if err != nil {
				return err
			}
		}

		return nil
	},
}

func init() {
	rootCmd.AddCommand(deleteCmd)
}
