package circular

import (
	"fmt"
)

type Buffer struct {
	buf     []byte
	maxSize int
}

func NewBuffer(size int) *Buffer {
	return &Buffer{buf: []byte{}, maxSize: size}
}

func (b *Buffer) ReadByte() (byte, error) {
	if len(b.buf) == 0 {
		return 0, fmt.Errorf("buffer empty")
	}
	c := b.buf[0]
	b.buf = b.buf[1:]
	return c, nil
}

func (b *Buffer) WriteByte(c byte) error {
	if len(b.buf) == b.maxSize {
		return fmt.Errorf("buffer full")
	}
	b.buf = append(b.buf, c)
	return nil
}

func (b *Buffer) Overwrite(c byte) {
	if len(b.buf) == b.maxSize {
		b.buf = b.buf[1:]
	}
	b.buf = append(b.buf, c)
}

func (b *Buffer) Reset() {
	b.buf = []byte{}
}
