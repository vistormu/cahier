package commandmanager

import (
	"fmt"
)

func Exec(user, host, command string, port, timeout int) error {
	cmd := "ssh -p %d %s@%s '%s'"
	cmd = fmt.Sprintf(cmd, port, user, host, command)

	_, err := RunCmd(cmd, timeout, true)
	if err != nil {
		return err
	}

	return nil
}

func Connect(user, host string, port int) error {
	cmd := fmt.Sprintf("ssh -X -tt -p %d %s@%s", port, user, host)

	_, err := RunCmd(cmd, 0, false)
	if err != nil {
		return err
	}

	return nil
}
