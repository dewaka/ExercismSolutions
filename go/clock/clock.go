package clock

import "fmt"

// Define the Clock type here.
type Clock struct {
	minutes int
}

func roundMinutes(m int) int {
	m %= 24 * 60
	if m < 0 {
		m += 24 * 60
	}

	return m
}

func fromMinutes(m int) Clock {
	return Clock{minutes: roundMinutes(m)}
}

func New(h, m int) Clock {
	return fromMinutes(h*60 + m)
}

func (c Clock) Add(m int) Clock {
	return fromMinutes(c.minutes + m)
}

func (c Clock) Subtract(m int) Clock {
	return fromMinutes(c.minutes - m)
}

func (c Clock) String() string {
	h := c.minutes / 60 % 24
	m := c.minutes % 60
	if m < 0 {
		m += 60
		h--
	}
	if h < 0 {
		h += 24
	}
	return fmt.Sprintf("%02d:%02d", h, m)
}
