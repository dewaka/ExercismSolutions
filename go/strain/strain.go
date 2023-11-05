package strain

// Implement the "Keep" and "Discard" function in this file.

// You will need typed parameters (aka "Generics") to solve this exercise.
// They are not part of the Exercism syllabus yet but you can learn about
// them here: https://go.dev/tour/generics/1

type predicate[T any] func(T) bool

func Keep[T any](a []T, p predicate[T]) []T {
	var r []T
	for _, e := range a {
		if p(e) {
			r = append(r, e)
		}
	}
	return r
}

func Discard[T any](a []T, p predicate[T]) []T {
	return Keep(a, func(t T) bool { return !p(t) })
}
