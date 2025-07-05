package commandmanager

import (
	"fmt"
)

func Bring(user, host, path string, port, timeout int) error {
	path, err := sanitizePath(path)

	cmd := fmt.Sprintf("scp -P %d -r %s@%s:%s .", port, user, host, path)
	_, err = RunCmd(cmd, timeout, true)
	if err != nil {
		return err
	}

	return nil
}

func Send(user, host, path, target string, port, timeout int) error {

	fmt.Println(path)
	fmt.Println(target)
	path, err := sanitizePath(path)
	if err != nil {
		return err
	}

	target, err = sanitizePath(target)
	if err != nil {
		return err
	}

	cmd := fmt.Sprintf("scp -P %d -r %s %s@%s:%s", port, path, user, host, target)
	_, err = RunCmd(cmd, timeout, true)
	if err != nil {
		return err
	}

	return nil
}

func Transfer(user1, host1, path1 string,
	user2, host2, path2 string, timeout int) error {

	var err error
	path1, err = sanitizePath(path1)
	if err != nil {
		return err
	}

	path2, err = sanitizePath(path2)
	if err != nil {
		return err
	}

	cmd := fmt.Sprintf("scp -3 -r %s@%s:%s %s@%s:%s",
		user1, host1, path1, user2, host2, path2)
	_, err = RunCmd(cmd, timeout, true)
	if err != nil {
		return err
	}

	return nil
}
