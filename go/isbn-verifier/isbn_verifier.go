package isbn

func IsValidISBN(isbn string) bool {
	var sum int
	multiplier := 10
	for _, c := range isbn {
		if c == '-' {
			continue
		}
		if c == 'X' && multiplier == 1 {
			sum += 10
		} else if c >= '0' && c <= '9' {
			sum += int(c-'0') * multiplier
		} else {
			return false
		}
		multiplier--
	}
	return sum%11 == 0 && multiplier == 0
}
