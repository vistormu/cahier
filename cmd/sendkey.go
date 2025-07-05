package cmd

import (
	"cahier/internal/commandmanager"
	"cahier/internal/pagemanager"
	"fmt"

	"github.com/spf13/cobra"
)

var sendKeyCmd = &cobra.Command{
	Use:   "sendkey",
	Short: "send SSH public key to remote server",
	Long:  `send SSH public key to a remote server for secure authentication. This command will copy the public key to the remote server's authorized_keys file.`,
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		nickname := args[0]
		if !pagemanager.SSHConnectionExists(nickname) {
			return fmt.Errorf("SSH connection with nickname '%s' does not exist", nickname)
		}

		connection, err := pagemanager.LoadSSHConnectionFromFile(nickname)
		if err != nil {
			return err
		}

		return commandmanager.SendKey(connection.User, connection.Host, connection.Port)
	},
}

func init() {
	rootCmd.AddCommand(sendKeyCmd)
}
