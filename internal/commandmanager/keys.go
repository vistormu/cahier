package commandmanager

import (
	"fmt"
)

func KeyGen() error {
	cmd := "ssh-keygen -t rsa -b 4096"
	_, err := RunCmd(cmd, 0, true)
	if err != nil {
		return err
	}

	return nil
}

func SendKey(user, host string, port int) error {
	cmd := fmt.Sprintf("ssh-copy-id -p %d %s@%s", port, user, host)

	_, err := RunCmd(cmd, 0, true)
	if err != nil {
		return err
	}

	return nil
}
