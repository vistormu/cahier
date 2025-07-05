package cmd

import (
	"cahier/internal/commandmanager"
	"cahier/internal/pagemanager"

	"github.com/spf13/cobra"
)

var (
	nPackets int
)

var pingCmd = &cobra.Command{
	Use:   "ping",
	Short: "ping an SSH connection",
	Long:  `ping an existing SSH connection by its nickname to check connectivity.`,
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		nickname := args[0]
		if !pagemanager.SSHConnectionExists(nickname) {
			return nil
		}

		connection, err := pagemanager.LoadSSHConnectionFromFile(nickname)
		if err != nil {
			return err
		}

		return commandmanager.Ping(connection.Host, nPackets)
	},
}

func init() {
	rootCmd.AddCommand(pingCmd)
	pingCmd.Flags().IntVarP(&nPackets, "packets", "p", 0, "number of packets to send")
}
