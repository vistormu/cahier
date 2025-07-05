package cmd

import (
	"cahier/internal/commandmanager"
	"cahier/internal/pagemanager"
	"fmt"

	"github.com/spf13/cobra"
)

var bringCmd = &cobra.Command{
	Use:   "bring",
	Short: "bring a file or directory to the current working directory",
	Long:  `bring a file or directory from a remote SSH connection to the current working directory using its nickname.`,
	Args:  cobra.ExactArgs(2),
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

		return commandmanager.Bring(connection.User, connection.Host, path, connection.Port, 5)
	},
}

func init() {
	rootCmd.AddCommand(bringCmd)
}
