// This is a "stub" file.  It's a little start on your solution.
// It's not a complete solution though; you have to write some code.

// Package acronym should have a package comment that summarizes what it's about.
// https://golang.org/doc/effective_go.html#commentary
package acronym

import "unicode"

// Abbreviate should have a comment documenting it.
func Abbreviate(s string) string {
	var acronym string
	var prev rune

	for _, r := range s {
		if r != '-' && unicode.IsPunct(r) {
			continue
		}
		if unicode.IsLetter(r) && !unicode.IsUpper(prev) && !unicode.IsLetter(prev) {
			acronym += string(unicode.ToUpper(r))
		}
		prev = r
	}

	return acronym
}
