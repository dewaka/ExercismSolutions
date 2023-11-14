package encode

import (
	"strconv"
	"strings"
	"unicode"
)

type pair struct {
	count int
	char  rune
}

func RunLengthEncode(input string) string {
	var stack []pair
	for _, c := range input {
		if len(stack) > 0 && stack[len(stack)-1].char == c {
			stack[len(stack)-1].count++
		} else {
			stack = append(stack, pair{1, c})
		}
	}
	b := strings.Builder{}
	for _, p := range stack {
		if p.count > 1 {
			b.WriteString(strconv.Itoa(p.count))
		}
		b.WriteString(string(p.char))
	}
	return b.String()
}

func RunLengthDecode(input string) string {
	res := strings.Builder{}
	n := strings.Builder{}
	for _, c := range input {
		if unicode.IsDigit(c) {
			n.WriteString(string(c))
		} else {
			if n.Len() > 0 {
				count, _ := strconv.Atoi(n.String())
				res.WriteString(strings.Repeat(string(c), count))
				n.Reset()
			} else {
				res.WriteString(string(c))
			}
		}
	}
	return res.String()
}
