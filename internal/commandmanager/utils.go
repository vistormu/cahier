package commandmanager

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	"github.com/google/shlex"
)

func RunCmd(cmdLine string, timeout int, capture bool) (string, error) {
	args, err := shlex.Split(cmdLine)
	if err != nil || len(args) == 0 {
		return "", fmt.Errorf("bad command: %w", err)
	}

	ctx := context.Background()
	if timeout > 0 {
		var cancel context.CancelFunc
		ctx, cancel = context.WithTimeout(ctx, time.Duration(timeout)*time.Second)
		defer cancel()
	}
	cmd := exec.CommandContext(ctx, args[0], args[1:]...)

	cmd.Stdin = os.Stdin

	if capture {
		var buf bytes.Buffer
		w := io.MultiWriter(os.Stdout, &buf)
		cmd.Stdout, cmd.Stderr = w, w
		err = cmd.Run()
		return buf.String(), err
	}

	cmd.Stdout, cmd.Stderr = os.Stdout, os.Stderr
	err = cmd.Run()
	return "", err
}

func sanitizePath(path string) (string, error) {
	if !filepath.IsAbs(path) {
		return path, nil
	}

	home, err := os.UserHomeDir()
	if err != nil {
		return path, err
	}

	home = filepath.Clean(home)
	path = filepath.Clean(path)

	rel, err := filepath.Rel(home, path)
	if err != nil || strings.HasPrefix(rel, "..") {
		return path, fmt.Errorf("%q is outside the home directory", path)
	}

	return rel, nil
}
