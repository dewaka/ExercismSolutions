package linkedlist

import "errors"

// Define List and Node types here.
// Note: The tests expect Node type to include an exported field with name Value to pass.

type Node struct {
	Value interface{}
	next  *Node
	prev  *Node
}

type List struct {
	front *Node
	back  *Node
}

func NewList(elements ...interface{}) *List {
	l := &List{}
	for _, e := range elements {
		l.Push(e)
	}
	return l
}

func (n *Node) Next() *Node {
	return n.next
}

func (n *Node) Prev() *Node {
	return n.prev
}

func (l *List) Unshift(v interface{}) {
	newNode := &Node{Value: v}
	if l.front == nil {
		l.front = newNode
		l.back = newNode
	} else {
		l.front.prev = newNode
		newNode.next = l.front
		l.front = newNode
	}
}

func (l *List) Push(v interface{}) {
	newNode := &Node{Value: v}
	if l.front == nil {
		l.front = newNode
		l.back = newNode
	} else {
		l.back.next = newNode
		newNode.prev = l.back
		l.back = newNode
	}
}

func (l *List) Shift() (interface{}, error) {
	if l.front == nil {
		return nil, errors.New("empty list")
	}
	v := l.front.Value
	l.front = l.front.next
	if l.front == nil {
		l.back = nil
	} else {
		l.front.prev = nil
	}
	return v, nil
}

func (l *List) Pop() (interface{}, error) {
	if l.back == nil {
		return nil, errors.New("empty list")
	}
	v := l.back.Value
	l.back = l.back.prev
	if l.back == nil {
		l.front = nil
	} else {
		l.back.next = nil
	}
	return v, nil
}

func (l *List) Reverse() {
	if l.front == nil {
		return
	}
	for n := l.front; n != nil; n = n.prev {
		n.next, n.prev = n.prev, n.next
	}
	l.front, l.back = l.back, l.front

}

func (l *List) First() *Node {
	return l.front
}

func (l *List) Last() *Node {
	return l.back
}
