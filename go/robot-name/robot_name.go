package robotname

import (
	"fmt"
	"math/rand"
	"time"
)

type Robot struct {
	name string
}

const maxRobotNames = 26 * 26 * 10 * 10 * 10

var (
	namePool = generateNames()
	idx      = 0
)

func generateNames() []string {
	pos := 0
	names := make([]string, maxRobotNames)

	for i := 'A'; i <= 'Z'; i++ {
		for j := 'A'; j <= 'Z'; j++ {
			for k := 0; k < 1000; k++ {
				names[pos] = fmt.Sprintf("%c%c%03d", i, j, k)
				pos++
			}
		}
	}

	rnd := rand.New(rand.NewSource(time.Now().UnixNano()))
	rnd.Shuffle(len(names), func(i, j int) {
		names[i], names[j] = names[j], names[i]
	})

	return names
}

func (r *Robot) Name() (string, error) {
	if r.name != "" {
		return r.name, nil
	}
	if idx >= len(namePool) {
		return "", fmt.Errorf("no more names available")
	}
	r.name = namePool[idx]
	idx++
	return r.name, nil
}

func (r *Robot) Reset() {
	r.name = ""
}
