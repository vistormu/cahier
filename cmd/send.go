package cmd

import (
	"cahier/internal/commandmanager"
	"cahier/internal/pagemanager"
	"fmt"

	"github.com/spf13/cobra"
)

var sendCmd = &cobra.Command{
	Use:   "send",
	Short: "send a file or directory to a remote SSH connection",
	Long:  `send a file or directory from the current working directory to a remote SSH connection using its nickname.`,
	Args:  cobra.ExactArgs(3),
	RunE: func(cmd *cobra.Command, args []string) error {
		nickname := args[0]
		if !pagemanager.SSHConnectionExists(nickname) {
			return fmt.Errorf("SSH connection with nickname '%s' does not exist", nickname)
		}

		connection, err := pagemanager.LoadSSHConnectionFromFile(nickname)
		if err != nil {
			return err
		}

		path := args[1]
		target := args[2]

		return commandmanager.Send(connection.User, connection.Host, path, target, connection.Port, 5)
	},
}

func init() {
	rootCmd.AddCommand(sendCmd)
}
