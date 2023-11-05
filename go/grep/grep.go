package grep

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

const (
	n = 1 << iota // Prepend the line number and a colon (':') to each line in the output, placing the number after the filename (if present).
	l = 1 << iota // Output only the names of the files that contain at least one matching line.
	i = 1 << iota // Match using a case-insensitive comparison.
	v = 1 << iota // Invert the program -- collect all lines that fail to match the pattern.
	x = 1 << iota // Search only for lines where the search string matches the entire line.
)

func Search(pattern string, flags, files []string) []string {
	opts, err := parseFlags(flags)
	if err != nil {
		panic(err)
	}

	var results []string
	multiple := len(files) > 1

	for _, file := range files {
		if res, err := searchFile(file, pattern, opts); err != nil {
			panic(err)
		} else {
			if multiple && !fileOnly(opts) {
				for _, r := range res {
					results = append(results, fmt.Sprintf("%s:%s", file, r))
				}
			} else {
				results = append(results, res...)
			}
		}
	}

	return results
}

func parseFlags(flags []string) (int, error) {
	var f int
	for _, flag := range flags {
		switch flag {
		case "-n":
			f |= n
		case "-l":
			f |= l
		case "-i":
			f |= i
		case "-v":
			f |= v
		case "-x":
			f |= x
		default:
			return 0, fmt.Errorf("invalid flag: %s", flag)
		}
	}
	return f, nil
}

func searchFile(file string, pat string, opts int) ([]string, error) {
	f, err := os.Open(file)
	if err != nil {
		return nil, err
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	line := 1
	var results []string

	for scanner.Scan() {
		s := scanner.Text()
		if invertMatch(opts) {
			if !lineMatches(s, pat, opts) {
				if fileOnly(opts) {
					return []string{file}, nil
				}
				results = append(results, formatResult(s, line, opts))
			}
		} else if lineMatches(s, pat, opts) {
			if fileOnly(opts) {
				return []string{file}, nil
			}
			results = append(results, formatResult(s, line, opts))
		}
		line++
	}

	return results, nil
}

func lineMatches(s string, pat string, opts int) bool {
	if ignoreCase(opts) {
		s = strings.ToLower(s)
		pat = strings.ToLower(pat)
	}
	if exactMatch(opts) {
		return s == pat
	}
	return strings.Contains(s, pat)
}

func exactMatch(opts int) bool {
	return opts&x != 0
}

func ignoreCase(opts int) bool {
	return opts&i != 0
}

func printLine(opts int) bool {
	return opts&n != 0
}

func fileOnly(opts int) bool {
	return opts&l != 0
}
func invertMatch(opts int) bool {
	return opts&v != 0
}

func formatResult(s string, line, opts int) string {
	if printLine(opts) {
		return fmt.Sprintf("%d:%s", line, s)
	} else {
		return s
	}
}
