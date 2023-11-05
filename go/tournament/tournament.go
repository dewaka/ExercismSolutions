package tournament

import (
	"bufio"
	"cmp"
	"fmt"
	"io"
	"slices"
	"strings"
)

type team struct {
	name string
	mp   int
	w    int
	d    int
	l    int
}

func (t *team) win() {
	t.mp++
	t.w++
}

func (t *team) draw() {
	t.mp++
	t.d++
}

func (t *team) loss() {
	t.mp++
	t.l++
}

func (team *team) points() int {
	return team.w*3 + team.d*1
}

type entry struct {
	wteam    string
	lteam    string
	decisive bool
}

type book map[string]*team

func (b book) addDefault(t string) {
	if _, ok := b[t]; !ok {
		b[t] = &team{name: t}
	}
}

func (b book) addEntry(e *entry) {
	if e == nil {
		return
	}

	b.addDefault(e.wteam)
	b.addDefault(e.lteam)

	if e.decisive {
		b[e.wteam].win()
		b[e.lteam].loss()
	} else {
		b[e.wteam].draw()
		b[e.lteam].draw()
	}
}

func readEntry(line string) (*entry, error) {
	sp := strings.Split(line, ";")
	if len(sp) != 3 {
		return nil, fmt.Errorf("invalid entry: %s", line)
	}

	var e entry
	switch sp[2] {
	case "win":
		e.wteam = sp[0]
		e.lteam = sp[1]
		e.decisive = true
	case "loss":
		e.wteam = sp[1]
		e.lteam = sp[0]
		e.decisive = true
	case "draw":
		e.wteam = sp[0]
		e.lteam = sp[1]
		e.decisive = false
	default:
		return nil, fmt.Errorf("invalid entry: %s", line)
	}

	return &e, nil
}

func updateEntries(b book, r io.Reader) error {
	scanner := bufio.NewScanner(r)
	for scanner.Scan() {
		line := scanner.Text()
		if strings.TrimSpace(line) == "" || strings.HasPrefix(line, "#") {
			continue
		}
		e, err := readEntry(line)
		if err != nil {
			return err
		}
		b.addEntry(e)
	}
	return nil
}

func sortedTeams(b book) []*team {
	sorted := make([]*team, 0, len(b))
	for _, v := range b {
		sorted = append(sorted, v)
	}

	slices.SortFunc(sorted, func(t1, t2 *team) int {
		if t1.points() != t2.points() {
			return cmp.Compare(t2.points(), t1.points())
		}
		return cmp.Compare(t1.name, t2.name)
	})

	return sorted
}

func bookResult(b book) string {
	var sb strings.Builder

	for _, v := range sortedTeams(b) {
		sb.WriteString(fmt.Sprintf("%-30s | %2d | %2d | %2d | %2d | %2d\n",
			v.name, v.mp, v.w, v.d, v.l, v.points()))
	}

	return sb.String()
}

func Tally(reader io.Reader, writer io.Writer) error {
	var b = make(book)

	if err := updateEntries(b, reader); err != nil {
		return err
	}

	fmt.Fprint(writer, "Team                           | MP |  W |  D |  L |  P\n")
	fmt.Fprint(writer, bookResult(b))
	return nil
}
