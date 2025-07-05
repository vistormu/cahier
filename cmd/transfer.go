package cmd

import (
	"cahier/internal/commandmanager"
	"cahier/internal/pagemanager"
	"fmt"

	"github.com/spf13/cobra"
)

var transferCmd = &cobra.Command{
	Use:   "transfer",
	Short: "transfer files from a remote server to another",
	Long:  `transfer files from a remote server to another using SSH. This command allows you to specify the source and destination paths for the transfer.`,
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		nickname1 := args[0]
		if !pagemanager.SSHConnectionExists(nickname1) {
			return fmt.Errorf("SSH connection with nickname1 '%s' does not exist", nickname1)
		}

		nickname2 := args[2]
		if !pagemanager.SSHConnectionExists(nickname2) {
			return fmt.Errorf("SSH connection with nickname2 '%s' does not exist", nickname2)
		}

		connection1, err := pagemanager.LoadSSHConnectionFromFile(nickname1)
		if err != nil {
			return err
		}

		connection2, err := pagemanager.LoadSSHConnectionFromFile(nickname2)
		if err != nil {
			return err
		}

		path1 := args[1]
		path2 := args[3]

		return commandmanager.Transfer(connection1.User, connection1.Host, path1, connection2.User, connection2.Host, path2, 5)
	},
}

func init() {
	rootCmd.AddCommand(transferCmd)
}
