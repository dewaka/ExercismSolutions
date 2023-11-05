package perfect

import (
	"fmt"
)

// Define the Classification type here.

type Classification int

const (
	ClassificationPerfect Classification = iota
	ClassificationAbundant
	ClassificationDeficient
)

var (
	ErrOnlyPositive = fmt.Errorf("not positive")
)

func aliquot(n int64) (int64, error) {
	if n <= 0 {
		return 0, ErrOnlyPositive
	}

	s := int64(1)
	for i := int64(2); i*i <= n; i++ {
		if n%i == 0 {
			s += i
			if n/i != i {
				s += n / i
			}
		}
	}

	return s, nil
}

func Classify(n int64) (Classification, error) {
	a, err := aliquot(n)

	if err != nil {
		return 0, err
	}

	switch {
	case n == 1 || a < n:
		return ClassificationDeficient, nil
	case a == n:
		return ClassificationPerfect, nil
	default:
		return ClassificationAbundant, nil
	}
}
