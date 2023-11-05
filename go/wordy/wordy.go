package wordy

import (
	"fmt"
	"strconv"
	"strings"
)

type number int
type operation byte

const (
	what operation = iota
	plus
	minus
	mul
	div
)

type atom struct {
	op  operation
	num number
}

func parse_atom(o operation, num string) (*atom, error) {
	n, err := strconv.Atoi(num)
	if err != nil {
		return nil, err
	}

	return &atom{o, number(n)}, nil
}

func evaluate(atoms []atom) (number, error) {
	var st []number
	var x number

	for _, a := range atoms {
		switch a.op {
		case what:
			st = append(st, a.num)
		case mul:
			if len(st) == 0 {
				return 0, fmt.Errorf("bad mul")
			}
			x, st = st[len(st)-1], st[:len(st)-1]
			st = append(st, x*a.num)
		case div:
			if len(st) == 0 {
				return 0, fmt.Errorf("bad div")
			}
			x, st = st[len(st)-1], st[:len(st)-1]
			st = append(st, x/a.num)
		case plus:
			if len(st) == 0 {
				return 0, fmt.Errorf("bad plus")
			}
			x, st = st[len(st)-1], st[:len(st)-1]
			st = append(st, x+a.num)
		case minus:
			if len(st) == 0 {
				return 0, fmt.Errorf("bad minus")
			}
			x, st = st[len(st)-1], st[:len(st)-1]
			st = append(st, x-a.num)
		}
	}

	if len(st) == 0 {
		return 0, nil
	} else {
		return st[len(st)-1], nil
	}
}

func tokenize(q string) ([]atom, error) {
	if !strings.HasSuffix(q, "?") {
		return nil, fmt.Errorf("wrong ending")
	}

	toks := strings.Split(q[:len(q)-1], " ")
	var atoms []atom
	n := len(toks)
	var tok string

	for i := 0; i < n; {
		tok = toks[i]

		if tok == "What" {
			if i+2 >= n || toks[i+1] != "is" {
				return nil, fmt.Errorf("bad what")
			}
			a, err := parse_atom(what, toks[i+2])
			if err != nil {
				return nil, err
			}
			atoms = append(atoms, *a)
			i += 3
		} else if tok == "plus" {
			if i+1 >= n {
				return nil, fmt.Errorf("bad plus")
			}
			a, err := parse_atom(plus, toks[i+1])
			if err != nil {
				return nil, err
			}
			atoms = append(atoms, *a)
			i += 2
		} else if tok == "minus" {
			if i+1 >= n {
				return nil, fmt.Errorf("bad minus")
			}
			a, err := parse_atom(minus, toks[i+1])
			if err != nil {
				return nil, err
			}
			atoms = append(atoms, *a)
			i += 2
		} else if tok == "multiplied" {
			if i+2 >= n || toks[i+1] != "by" {
				return nil, fmt.Errorf("bad multi")
			}
			a, err := parse_atom(mul, toks[i+2])
			if err != nil {
				return nil, err
			}
			atoms = append(atoms, *a)
			i += 3
		} else if tok == "divided" {
			if i+2 >= n || toks[i+1] != "by" {
				return nil, fmt.Errorf("bad multi")
			}
			a, err := parse_atom(div, toks[i+2])
			if err != nil {
				return nil, err
			}
			atoms = append(atoms, *a)
			i += 3
		} else {
			return nil, fmt.Errorf("unexpected token")
		}
	}
	return atoms, nil
}

func Answer(question string) (int, bool) {
	atoms, err := tokenize(question)
	if err != nil {
		return 0, false
	}
	res, err := evaluate(atoms)
	if err != nil {
		return 0, false
	}

	return int(res), true
}
