package cipher

import "fmt"

// Define the shift and vigenere types here.
// Both types should satisfy the Cipher interface.

type shift string
type vigenere string

func NewCaesar() Cipher {
	return NewShift(0)
}

func NewShift(distance int) Cipher {
	panic("Please implement the NewShift function")
}

func (c shift) Encode(input string) string {
	fmt.Printf("c=%s and input=%s\n", c, input)
	panic("Please implement the Encode function")
}

func (c shift) Decode(input string) string {
	panic("Please implement the Decode function")
}

func NewVigenere(key string) Cipher {
	panic("Please implement the NewVigenere function")
}

func (v vigenere) Encode(input string) string {
	panic("Please implement the Encode function")
}

func (v vigenere) Decode(input string) string {
	panic("Please implement the Decode function")
}
