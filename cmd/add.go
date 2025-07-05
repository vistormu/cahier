package cmd

import (
	// "cahier/internal/commandmanager"
	"cahier/internal/pagemanager"
	"cahier/internal/prompter"
	"fmt"

	"github.com/spf13/cobra"
)

var addCmd = &cobra.Command{
	Use:   "add",
	Short: "add a new SSH connection",
	Long:  `add a new SSH connection with a nickname, host, IP address, and port.`,
	Args:  cobra.ArbitraryArgs,
	RunE: func(cmd *cobra.Command, args []string) error {
		var nickname string
		var hostString string
		var err error

		if len(args) == 0 {
			nickname = prompter.AskForNickname()
			user := prompter.AskForUser()
			host := prompter.AskForHost()
			port := prompter.AskForPort()
			hostString = user + "@" + host + ":" + port
		} else if len(args) == 1 {
			nickname = args[0]
			user := prompter.AskForUser()
			host := prompter.AskForHost()
			port := prompter.AskForPort()
			hostString = user + "@" + host + ":" + port
		} else {
			nickname = args[0]
			hostString = args[1]
		}

		if pagemanager.SSHConnectionExists(nickname) {
			if prompter.AskForEdit() {
				connection, err := pagemanager.NewSSHConnection(nickname, hostString)
				if err != nil {
					return err
				}
				err = pagemanager.EditSSHConnectionFromFile(nickname, connection)
				if err != nil {
					return err
				}

				fmt.Printf("SSH connection '%s' updated successfully!\n", nickname)
			}

			return nil
		}

		connection, err := pagemanager.NewSSHConnection(nickname, hostString)
		if err != nil {
			return err
		}

		err = pagemanager.SaveSSHConnectionToFile(connection)
		if err != nil {
			return err
		}

		fmt.Printf("SSH connection '%s' added successfully!\n", connection.Nickname)

		// try to send the public key
		// commandmanager.SendKey(connection.User, connection.Host, connection.Port)

		return nil
	},
}

func init() {
	rootCmd.AddCommand(addCmd)
}
