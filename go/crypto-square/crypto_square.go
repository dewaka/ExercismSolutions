package cryptosquare

import (
	"math"
	"unicode"
)

type Cipher struct {
	plainText  []rune
	cols, rows int
}

func NewCipher(pt string) *Cipher {
	norm := normalize(pt)
	c, r := size(norm)
	return &Cipher{norm, c, r}
}

func (c *Cipher) runeAt(row, col int) rune {
	if row*c.cols+col >= len(c.plainText) {
		return ' '
	}
	return c.plainText[row*c.cols+col]
}

func (c *Cipher) Encode() string {
	var res string

	for i := 0; i < c.cols; i++ {
		for j := 0; j < c.rows; j++ {
			res += string(c.runeAt(j, i))
		}
		res += " "
	}

	if len(res) == 0 {
		return res
	}

	return res[:len(res)-1]
}

func Encode(pt string) string {
	c := NewCipher(pt)
	return c.Encode()
}

func size(norm []rune) (int, int) {
	n := len(norm)
	r := math.Ceil(math.Sqrt(float64(n)))
	c := int(math.Ceil(float64(n) / r))
	if c < int(r) {
		return int(r), c
	}
	return c, int(r)
}

func normalize(p string) []rune {
	var norm []rune
	for _, r := range p {
		if unicode.IsLetter(r) || unicode.IsDigit(r) {
			norm = append(norm, unicode.ToLower(r))
		}
	}
	return norm
}
