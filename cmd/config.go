package cmd

import (
	"os"
	"os/exec"

	"cahier/internal/core"

	"github.com/spf13/cobra"
)

var configCmd = &cobra.Command{
	Use:   "config",
	Short: "open Cahier configuration",
	Long:  `Open the Cahier configuration file in the default editor for editing.`,
	Args:  cobra.NoArgs,
	RunE: func(cmd *cobra.Command, args []string) error {
		editor := os.Getenv("EDITOR")
		if editor == "" {
			editor = "vi"
		}

		// open core.CahierDirectory in the default editor
		cmdEditor := exec.Command(editor, core.CahierDirectory)
		cmdEditor.Stdout = os.Stdout
		cmdEditor.Stderr = os.Stderr
		if err := cmdEditor.Run(); err != nil {
			return err
		}

		return nil
	},
}

func init() {
	rootCmd.AddCommand(configCmd)
}
