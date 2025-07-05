package prompter

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/vistormu/go-dsa/ansi"
)

func AskForEdit() bool {
	reader := bufio.NewReader(os.Stdin)
	prompt := fmt.Sprintf("%sa connection with the same nickname was found%s\n%s do you want to overwrite the connection? (y/[n]):%s ",
		ansi.Red,
		ansi.Reset,
		ansi.Yellow+EditIcon+ansi.Reset+ansi.Bold+ansi.Blue,
		ansi.Reset,
	)

	for {
		fmt.Print(prompt)
		input, err := reader.ReadString('\n')
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error reading input: %v\n", err)
			continue
		}
		switch strings.TrimSpace(strings.ToLower(input)) {
		case "y", "yes":
			return true
		case "n", "no":
			return false
		default:
			return false
		}
	}
}

func AskForNickname() string {
	reader := bufio.NewReader(os.Stdin)
	prompt := fmt.Sprintf("%s %senter nickname:%s ",
		ansi.Yellow+NicknameIcon+ansi.Reset,
		ansi.Bold+ansi.Blue,
		ansi.Reset,
	)

	for {
		fmt.Print(prompt)
		input, err := reader.ReadString('\n')
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error reading nickname: %v\n", err)
			continue
		}
		nickname := strings.TrimSpace(input)
		if nickname == "" {
			fmt.Println("Nickname cannot be empty. Please try again.")
			continue
		}

		// everything is ok
		fmt.Printf("%s%s %senter nickname:%s %s %s%s%s\n",
			ansi.Up+ansi.LineEnd,
			ansi.Yellow+NicknameIcon+ansi.Reset,
			ansi.Bold+ansi.Blue,
			ansi.Reset,
			nickname,
			ansi.Green,
			CheckIcon,
			ansi.Reset,
		)

		return nickname
	}
}

func AskForUser() string {
	reader := bufio.NewReader(os.Stdin)
	prompt := fmt.Sprintf("%s %senter user:%s ",
		ansi.Yellow+UserIcon+ansi.Reset,
		ansi.Bold+ansi.Blue,
		ansi.Reset,
	)

	for {
		fmt.Print(prompt)
		input, err := reader.ReadString('\n')
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error reading user: %v\n", err)
			continue
		}
		user := strings.TrimSpace(input)
		if user == "" {
			fmt.Println("User cannot be empty. Please try again.")
			continue
		}

		// everything is ok
		fmt.Printf("%s%s %senter user:%s %s %s%s%s\n",
			ansi.Up+ansi.LineEnd,
			ansi.Yellow+UserIcon+ansi.Reset,
			ansi.Bold+ansi.Blue,
			ansi.Reset,
			user,
			ansi.Green,
			CheckIcon,
			ansi.Reset,
		)

		return user
	}
}

func AskForHost() string {
	reader := bufio.NewReader(os.Stdin)
	prompt := fmt.Sprintf("%s %senter host (IP address or domain):%s ",
		ansi.Yellow+HostIcon+ansi.Reset,
		ansi.Bold+ansi.Blue,
		ansi.Reset,
	)

	for {
		fmt.Print(prompt)
		input, err := reader.ReadString('\n')
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error reading host: %v\n", err)
			continue
		}
		host := strings.TrimSpace(input)
		if host == "" {
			fmt.Println("Host cannot be empty. Please try again.")
			continue
		}

		// everything is ok
		fmt.Printf("%s%s %senter host:%s %s %s%s%s\n",
			ansi.Up+ansi.LineEnd,
			ansi.Yellow+HostIcon+ansi.Reset,
			ansi.Bold+ansi.Blue,
			ansi.Reset,
			host,
			ansi.Green,
			CheckIcon,
			ansi.Reset,
		)

		return host
	}
}

func AskForPort() string {
	reader := bufio.NewReader(os.Stdin)
	prompt := fmt.Sprintf("%s %senter port (default 22):%s ",
		ansi.Yellow+PortIcon+ansi.Reset,
		ansi.Bold+ansi.Blue,
		ansi.Reset,
	)

	for {
		fmt.Print(prompt)
		input, err := reader.ReadString('\n')
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error reading port: %v\n", err)
			continue
		}
		port := strings.TrimSpace(input)
		if port == "" {
			port = "22" // default port
		}

		// everything is ok
		fmt.Printf("%s%s %senter port:%s %s %s%s%s\n",
			ansi.Up+ansi.LineEnd,
			ansi.Yellow+PortIcon+ansi.Reset,
			ansi.Bold+ansi.Blue,
			ansi.Reset,
			port,
			ansi.Green,
			CheckIcon,
			ansi.Reset,
		)

		return port
	}
}

func AskForConfirmClear() bool {
	reader := bufio.NewReader(os.Stdin)
	prompt := fmt.Sprintf("%s %sAre you sure you want to clear all SSH connections? (y/[n]):%s ",
		ansi.Yellow+EditIcon+ansi.Reset,
		ansi.Bold+ansi.Blue,
		ansi.Reset,
	)

	for {
		fmt.Print(prompt)
		input, err := reader.ReadString('\n')
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error reading input: %v\n", err)
			continue
		}
		switch strings.TrimSpace(strings.ToLower(input)) {
		case "y", "yes":
			return true
		case "n", "no":
			return false
		default:
			return false
		}
	}
}

func AskForConfirmDelete(nickname string) bool {
	reader := bufio.NewReader(os.Stdin)
	prompt := fmt.Sprintf("%s %sAre you sure you want to delete the SSH connection %s%s%s? (y/[n]):%s ",
		ansi.Yellow+DeleteIcon+ansi.Reset,
		ansi.Bold+ansi.Blue,
		ansi.Reset+ansi.Magenta,
		nickname,
		ansi.Bold+ansi.Blue,
		ansi.Reset,
	)

	for {
		fmt.Print(prompt)
		input, err := reader.ReadString('\n')
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error reading input: %v\n", err)
			continue
		}
		switch strings.TrimSpace(strings.ToLower(input)) {
		case "y", "yes":
			return true
		case "n", "no":
			return false
		default:
			return false
		}
	}
}
