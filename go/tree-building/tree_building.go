package tree

import (
	"fmt"
	"sort"
)

type Record struct {
	ID     int
	Parent int
	// feel free to add fields as you see fit
}

type Node struct {
	ID       int
	Children []*Node
	// feel free to add fields as you see fit
}

func addChild(rs map[int]*Node, r *Record) error {
	if r.ID == 0 && r.Parent != r.ID {
		return fmt.Errorf("bad parent for root node")
	}
	if rs[r.ID] != nil {
		return fmt.Errorf("duplicate node")
	}

	if r.ID != 0 && rs[r.Parent] == nil {
		rs[r.Parent] = newNode(r.Parent)
	}
	node := rs[r.ID]
	if node == nil {
		node = newNode(r.ID)
	}
	if r.ID != 0 {
		p := rs[r.Parent]
		p.Children = append(rs[r.Parent].Children, node)
		sort.Slice(p.Children, func(i, j int) bool {
			return p.Children[i].ID < p.Children[j].ID
		})
	}
	return nil
}

func newNode(id int) *Node {
	return &Node{id, nil}
}

func Build(records []Record) (*Node, error) {
	rs := make(map[int]*Node)

	rootFound := false

	for _, r := range records {
		if r.ID == 0 {
			rootFound = true
		}
		if err := addChild(rs, &r); err != nil {
			return nil, err
		}
	}
	if len(rs) == 0 {
		return nil, nil
	}
	if !rootFound {
		return nil, fmt.Errorf("no root node")
	} else {
		return rs[0], nil
	}
}
