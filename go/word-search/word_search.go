package wordsearch

import (
	"fmt"
	"strings"
)

type Point struct {
	r int
	i int
}

func findInRow(word string, row string) int {
	return strings.Index(row, word)
}

func columnToRow(puzzle []string) []string {
	var result []string
	for i := 0; i < len(puzzle[0]); i++ {
		var row string
		for j := 0; j < len(puzzle); j++ {
			row += string(puzzle[j][i])
		}
		result = append(result, row)
	}
	return result
}

func findInRows(word string, rows []string) *Point {
	for i, row := range rows {
		index := findInRow(word, row)
		if index != -1 {
			return &Point{i, index}
		}
	}
	return nil
}

func reverseWord(word string) string {
	var result string
	for _, letter := range word {
		result = string(letter) + result
	}
	return result
}

func Solve(words []string, puzzle []string) (map[string][2][2]int, error) {
	result := make(map[string][2][2]int)
	var err error
	var p *Point

	for _, word := range words {
		p = findInRows(word, puzzle)
		if p != nil {
			result[word] = [2][2]int{{p.i, p.r}, {p.i + len(word) - 1, p.r}}
			continue
		}
		p = findInRows(reverseWord(word), puzzle)
		if p != nil {
			result[word] = [2][2]int{{p.i + len(word) - 1, p.r}, {p.i, p.r}}
		}
	}

	columns := columnToRow(puzzle)
	for _, word := range words {
		p = findInRows(word, columns)
		if p != nil {
			result[word] = [2][2]int{{p.r, p.i}, {p.r, p.i + len(word) - 1}}
			continue
		}
		p = findInRows(reverseWord(word), columns)
		if p != nil {
			result[word] = [2][2]int{{p.r, p.i + len(word) - 1}, {p.r, p.i}}
		}
	}

	fmt.Println(result)

	for _, word := range words {
		_, ok := result[word]
		if !ok {
			result[word] = [2][2]int{{-1, -1}, {-1, -1}}
			err = fmt.Errorf("word '%s' not found", word)
		}
	}

	return result, err
}
