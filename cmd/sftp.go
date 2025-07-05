package cmd

import (
	"cahier/internal/commandmanager"
	"cahier/internal/pagemanager"
	"fmt"

	"github.com/spf13/cobra"
)

var sftpCmd = &cobra.Command{
	Use:   "sftp",
	Short: "start an SFTP session",
	Long:  `start an SFTP session on a remote SSH connection using its nickname. This allows you to transfer files securely over SSH.`,
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

		return commandmanager.Sftp(connection.User, connection.Host, connection.Port, 0)
	},
}

func init() {
	rootCmd.AddCommand(sftpCmd)
}
