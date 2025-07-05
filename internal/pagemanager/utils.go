package pagemanager

import (
	"strings"
)

func sanitizeNickname(nickname string) string {
	// sanitize nickname to remove any unwanted characters
	nickname = strings.Map(func(r rune) rune {
		if r >= 'a' && r <= 'z' || r >= 'A' && r <= 'Z' || r >= '0' && r <= '9' || r == '-' || r == '_' {
			return r
		}
		return -1 // remove unwanted characters
	}, nickname)

	nickname = strings.ToLower(nickname)
	nickname = strings.ReplaceAll(nickname, " ", "-")

	return nickname
}
