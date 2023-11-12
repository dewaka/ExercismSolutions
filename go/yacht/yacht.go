package yacht

import "slices"

func Score(dice []int, category string) int {
	if len(dice) != 5 {
		return 0
	}

	switch category {
	case "ones":
		return 1 * countOf(1, dice)
	case "twos":
		return 2 * countOf(2, dice)
	case "threes":
		return 3 * countOf(3, dice)
	case "fours":
		return 4 * countOf(4, dice)
	case "fives":
		return 5 * countOf(5, dice)
	case "sixes":
		return 6 * countOf(6, dice)
	case "full house":
		return fullHouse(dice)
	case "four of a kind":
		return fourOfAKind(dice)
	case "little straight":
		return littleStraight(dice)
	case "big straight":
		return bigStraight(dice)
	case "choice":
		return sum(dice)
	case "yacht":
		return yacht(dice)
	}
	return 0
}

type counter [7]int

func countOf(d int, dice []int) int {
	c := 0
	for _, v := range dice {
		if v == d {
			c++
		}
	}
	return c
}

func sum(dice []int) int {
	s := 0
	for _, v := range dice {
		s += v
	}
	return s
}

func bigStraight(dice []int) int {
	slices.Sort(dice)
	for i := 0; i < 5; i++ {
		if dice[i] != i+2 {
			return 0
		}
	}
	return 30
}

func littleStraight(dice []int) int {
	slices.Sort(dice)
	for i := 0; i < 5; i++ {
		if dice[i] != i+1 {
			return 0
		}
	}
	return 30
}

func fourOfAKind(dice []int) int {
	c := countDice(dice)
	for i := 1; i < 7; i++ {
		if c[i] >= 4 {
			return 4 * i
		}
	}
	return 0
}

func countDice(dice []int) counter {
	c := counter{}
	for _, v := range dice {
		c[v]++
	}
	return c
}

func yacht(dice []int) int {
	c := countDice(dice)
	for i := 1; i < 7; i++ {
		if c[i] == 5 {
			return 50
		}
	}
	return 0
}

func fullHouse(dice []int) int {
	c := countDice(dice)
	for i := 1; i < 7; i++ {
		if c[i] == 3 {
			for j := 1; j < 7; j++ {
				if c[j] == 2 {
					return 3*i + 2*j
				}
			}
		}
	}
	return 0
}
