package linkedlist
import (
	"errors"
)
// Define List and Node types here.
// Note: The tests expect Node type to include an exported field with name Value to pass.
type List struct {
	head *Node
	tail *Node
}
type Node struct {
	Value interface{}
	next  *Node
	prev  *Node
}

func NewList(elements ...interface{}) *List {
	var list *List = new(List)

	for _, Value := range elements {
		var node *Node = new(Node)
		node.Value = Value
		if list.head == nil {
			list.head = node
			list.tail = node
		} else {
			list.tail.next = node
			node.prev = list.tail
			list.tail = node
		}

	}
	return list
}

func (n *Node) Next() *Node {
	return n.next
}

func (n *Node) Prev() *Node {
	return n.prev
}

func (l *List) Unshift(v interface{}) {
	if l.head == nil {
		var node *Node = new(Node)
		node.Value = v
		l.head = node
		l.tail = node
		return
	}
	var node *Node = new(Node)
	node.Value = v
	l.head.prev = node
	node.next = l.head
	l.head = node
}

func (l *List) Push(v interface{}) {
	if l.head == nil {
		var node *Node = new(Node)
		node.Value = v
		l.head = node
		l.tail = node
		return
	}
	var node *Node = new(Node)
	node.Value = v
	l.tail.next = node
	node.prev = l.tail
	l.tail = node
}

func (l *List) Shift() (interface{}, error) {
	if l.head == nil {
		return -1, errors.New("List empty!!")
	}
	var t *Node = l.head
	if l.head.next == nil {
		l.head = l.head.next
        l.tail = l.head
		return t.Value, nil
	}
	l.head = l.head.next
	l.head.prev = nil
	return t.Value, nil
}

func (l *List) Pop() (interface{}, error) {
	if l.tail == nil {
		return -1, errors.New("List empty!!")
	}
	var t *Node = l.tail
	if l.tail.prev == nil {
		l.tail = l.tail.prev
        l.head = l.tail
		return t.Value, nil
	}
	l.tail = l.tail.prev
	l.tail.next = nil
	return t.Value, nil
}

func (l *List) Reverse() {
	if l.head == nil {
		return
	}
	l.head = l.tail
	var temp *Node = l.tail.next
	for l.tail != nil {
		l.tail.next = l.tail.prev
		l.tail.prev = temp
		temp = l.tail
		l.tail = l.tail.next
	}
	l.tail = temp
}

func (l *List) First() *Node {
	return l.head
}

func (l *List) Last() *Node {
	return l.tail
}