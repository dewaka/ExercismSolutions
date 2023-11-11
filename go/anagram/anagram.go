package anagram

import (
	"sort"
	"strings"
)

func Detect(subject string, candidates []string) []string {
	var anagrams []string
	ns := normalize(subject)
	for _, candidate := range candidates {
		if strings.ToLower(candidate) != strings.ToLower(subject) && ns == normalize(candidate) {
			anagrams = append(anagrams, candidate)
		}
	}
	return anagrams
}

func normalize(s string) string {
	s = strings.ToLower(s)
	return sortString(s)
}

func sortString(s string) string {
	w := strings.Split(s, "")
	sort.Strings(w)
	return strings.Join(w, "")
}
