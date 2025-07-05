package cmd

import (
	"fmt"
	"strings"

	"cahier/internal/pagemanager"

	"github.com/spf13/cobra"
	"github.com/vistormu/go-dsa/ansi"
)

var listCmd = &cobra.Command{
	Use:   "list",
	Short: "list all SSH connections",
	Long:  `list all SSH connections stored in the Cahier page manager.`,
	Args:  cobra.NoArgs,
	RunE: func(cmd *cobra.Command, args []string) error {
		connections, err := pagemanager.LoadAllSSHConnections()
		if err != nil {
			return err
		}

		if len(connections) == 0 {
			return nil
		}

		// get the nicknames max length
		nicknamesMaxLength := 0
		for _, connection := range connections {
			nicknamesMaxLength = max(nicknamesMaxLength, len(connection.Nickname))
		}

		lines := make([]string, len(connections)+1)
		// print the header
		lines[0] = fmt.Sprintf("%-*s   %s",
			nicknamesMaxLength,
			ansi.Underline+"nickname"+ansi.Reset,
			ansi.Underline+"uri"+ansi.Reset,
		)

		// print the output
		for i, connection := range connections {
			nickname := fmt.Sprintf("%s%-*s%s",
				ansi.Bold+ansi.Green,
				nicknamesMaxLength,
				connection.Nickname,
				ansi.Reset,
			)

			uriStr := fmt.Sprintf("%s%s%s@%s:%s%d%s",
				ansi.Bold+ansi.Cyan,
				connection.User,
				ansi.Reset,
				ansi.Blue+connection.Host+ansi.Reset,
				ansi.Yellow, connection.Port, ansi.Reset,
			)

			lines[i+1] = nickname + "   " + uriStr
		}

		fmt.Println(strings.Join(lines, "\n") + "\n")

		return nil
	},
}

func init() {
	rootCmd.AddCommand(listCmd)
}
