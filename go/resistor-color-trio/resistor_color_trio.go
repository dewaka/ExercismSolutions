package resistorcolortrio

import (
	"fmt"
	"math"
)

var colors = []string{"black", "brown", "red", "orange", "yellow", "green", "blue", "violet", "grey", "white"}

func index(color string) int {
	for i, c := range colors {
		if c == color {
			return i
		}
	}
	return -1
}

func calculate(colors []string) int {
	if len(colors) < 3 {
		return 0
	}
	ohms := (index(colors[0])*10 + index(colors[1])) * int(math.Pow10(index(colors[2])))
	return ohms
}

func format(ohms int) string {
	var unit string
	switch {
	case ohms >= 1000*1000*1000:
		ohms /= 1000 * 1000 * 1000
		unit = " gigaohms"
	case ohms >= 1000*1000:
		ohms /= 1000 * 1000
		unit = " megaohms"
	case ohms >= 1000:
		ohms /= 1000
		unit = " kiloohms"
	default:
		unit = " ohms"
	}
	return fmt.Sprintf("%d%s", ohms, unit)
}

// Label describes the resistance value given the colors of a resistor.
// The label is a string with a resistance value with an unit appended
// (e.g. "33 ohms", "470 kiloohms").
func Label(colors []string) string {
	ohms := calculate(colors)
	return format(ohms)
}
