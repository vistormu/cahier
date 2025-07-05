package commandmanager

import (
	"fmt"
)

func Ping(host string, times int) error {
	var cmd string
	if times > 1 {
		cmd = fmt.Sprintf("ping -c %d %s", times, host)
	} else {
		cmd = fmt.Sprintf("ping %s", host)
	}

	_, err := RunCmd(cmd, 0, true)
	if err != nil {
		return err
	}

	return nil
}
