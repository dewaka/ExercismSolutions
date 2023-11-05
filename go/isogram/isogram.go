package isogram

import (
	"strings"
	"unicode"
)

func version1(word string) bool {
	seen := make(map[rune]bool)
	for _, c := range word {
		switch c {
		case '-', ' ':
			continue
		default:
			l := unicode.ToLower(c)
			if seen[l] {
				return false
			}
			seen[l] = true
		}

	}

	return true

}

func version2(word string) bool {
	s := strings.ToLower(word)

	for i, c := range s {
		if unicode.IsLetter(c) && strings.ContainsRune(s[i+1:], c) {
			return false
		}
	}

	return true
}

func IsIsogram(word string) bool {
	return version1(word)
}
