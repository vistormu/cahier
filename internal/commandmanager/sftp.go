package commandmanager

import (
	"fmt"
)

func Sftp(user, host string, port, timeout int) error {
	cmd := "sftp -P %d %s@%s"
	cmd = fmt.Sprintf(cmd, port, user, host)

	_, err := RunCmd(cmd, timeout, false)
	if err != nil {
		return err
	}

	return nil
}
