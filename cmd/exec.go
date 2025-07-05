package cmd

import (
	"cahier/internal/commandmanager"
	"cahier/internal/pagemanager"
	"fmt"

	"github.com/spf13/cobra"
)

var execCmd = &cobra.Command{
	Use:   "exec",
	Short: "execute a command on a remote SSH connection",
	Long:  `execute a command on a remote SSH connection using its nickname. The command will be executed in the context of the remote user.`,
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

		command := args[1]
		return commandmanager.Exec(connection.User, connection.Host, command, connection.Port, 5)
	},
}

func init() {
	rootCmd.AddCommand(execCmd)
}
