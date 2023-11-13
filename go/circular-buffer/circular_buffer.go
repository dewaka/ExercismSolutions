package circular

import (
	"fmt"
)

type Buffer struct {
	buf        []byte
	head, tail int
	full       bool
}

func NewBuffer(size int) *Buffer {
	return &Buffer{buf: make([]byte, size), head: 0, tail: 0, full: false}
}

func (b *Buffer) ReadByte() (byte, error) {
	if b.head == b.tail && !b.full {
		return 0, fmt.Errorf("buffer empty")
	}
	c := b.buf[b.head]
	b.head = (b.head + 1) % len(b.buf)
	b.full = false
	return c, nil
}

func (b *Buffer) WriteByte(c byte) error {
	if b.full {
		return fmt.Errorf("buffer full")
	}
	b.buf[b.tail] = c
	b.tail = (b.tail + 1) % len(b.buf)
	if b.tail == b.head {
		b.full = true
	}
	return nil
}

func (b *Buffer) Overwrite(c byte) {
	if b.full {
		b.head = (b.head + 1) % len(b.buf)
	}
	b.buf[b.tail] = c
	b.tail = (b.tail + 1) % len(b.buf)
	if b.tail == b.head {
		b.full = true
	}
}

func (b *Buffer) Reset() {
	b.head = 0
	b.tail = 0
	b.full = false
}
