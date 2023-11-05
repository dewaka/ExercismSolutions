package stringset

import (
	"fmt"
	"strings"
)

// Implement Set as a collection of unique string values.
//
// For Set.String, use '{' and '}', output elements as double-quoted strings
// safely escaped with Go syntax, and use a comma and a single space between
// elements. For example, a set with 2 elements, "a" and "b", should be formatted as {"a", "b"}.
// Format the empty set as {}.

// Define the Set type here.

type Set map[any]bool

func New() Set {
	return make(Set)
}

func NewFromSlice(l []string) Set {
	s := New()
	for _, v := range l {
		s[v] = true
	}
	return s
}

func (s Set) String() string {
	items := make([]string, 0, len(s))
	for k := range s {
		items = append(items, fmt.Sprintf("%q", k))
	}
	return "{" + strings.Join(items, ", ") + "}"
}

func (s Set) IsEmpty() bool {
	return len(s) == 0
}

func (s Set) Has(elem string) bool {
	_, ok := s[elem]
	return ok
}

func (s Set) Add(elem string) {
	s[elem] = true
}

func Subset(s1, s2 Set) bool {
	for k := range s1 {
		if !s2[k] {
			return false
		}
	}
	return true
}

func Disjoint(s1, s2 Set) bool {
	for k := range s1 {
		if s2[k] {
			return false
		}
	}
	return true
}

func Equal(s1, s2 Set) bool {
	return Subset(s1, s2) && Subset(s2, s1)
}

func Intersection(s1, s2 Set) Set {
	res := New()
	for k := range s1 {
		if s2[k] {
			res[k] = true
		}
	}
	return res
}

func Difference(s1, s2 Set) Set {
	res := New()
	for k := range s1 {
		if !s2[k] {
			res[k] = true
		}
	}
	return res
}

func Union(s1, s2 Set) Set {
	res := New()
	for k := range s1 {
		res[k] = true
	}
	for k := range s2 {
		res[k] = true
	}
	return res
}
