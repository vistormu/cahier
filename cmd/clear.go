package cmd

import (
	"cahier/internal/pagemanager"
	"cahier/internal/prompter"
	"fmt"

	"github.com/spf13/cobra"
)

var clearCmd = &cobra.Command{
	Use:   "clear",
	Short: "clear all SSH connections",
	Long:  `clear all SSH connections from the Cahier page manager.`,
	Args:  cobra.NoArgs,
	RunE: func(cmd *cobra.Command, args []string) error {
		if prompter.AskForConfirmClear() {
			err := pagemanager.ClearSSHConnections()
			if err != nil {
				return err
			}

			fmt.Println("All SSH connections cleared successfully!")
		}

		return nil
	},
}

func init() {
	rootCmd.AddCommand(clearCmd)
}
