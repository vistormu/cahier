package core

import (
	"os"
)

var CahierDirectory = "~/.cahier"

func init() {
	dir, err := ExpandTilde(CahierDirectory)
	if err != nil {
		panic("failed to expand home directory: " + err.Error())
	}
	CahierDirectory = dir

	if _, err := os.Stat(CahierDirectory); os.IsNotExist(err) {
		if err := os.MkdirAll(CahierDirectory, 0o755); err != nil {
			panic("failed to create Cahier directory: " + err.Error())
		}
	}
}
