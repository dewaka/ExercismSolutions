package lsproduct

import "errors"

func toNums(digits string) ([]int, error) {
	var nums []int = make([]int, len(digits))
	for i, c := range digits {
		if c < '0' || c > '9' {
			return nil, errors.New("digits input must only contain digits")
		}
		nums[i] = int(c - '0')
	}
	return nums, nil
}

func LargestSeriesProduct(digits string, span int) (int64, error) {
	if span < 0 {
		return 0, errors.New("span must not be negative")
	}
	nums, err := toNums(digits)
	if err != nil {
		return 0, err
	}
	if span > len(nums) {
		return 0, errors.New("span must be smaller than string length")
	}

	var max int64 = 0
	for i := 0; i <= len(nums)-span; i++ {
		var prod int64 = 1
		for j := 0; j < span; j++ {
			prod *= int64(nums[i+j])
		}
		if prod > max {
			max = prod
		}
	}

	return max, nil
}
