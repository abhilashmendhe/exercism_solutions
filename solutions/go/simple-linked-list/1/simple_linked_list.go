package linkedlist

import (
    "errors"
)
// Define the List and Element types here.
type Node struct {
	data int
	next *Node
}
type List struct {
	size int
	tail *Node
	head *Node
}
func New(elements []int) *List {

	var tempList *List = new(List)
	for _, v := range elements {

		// Create a node
		var tnode *Node = new(Node)
		tnode.data = v
		tnode.next = nil

		// Check if list head is nill
		if tempList.head == nil {
			tempList.head = tnode
			tempList.tail = tnode
		} else {
			tempList.tail.next = tnode
			tempList.tail = tnode
		}
		tempList.size++
	}

	return tempList
}

func (l *List) Size() int {
	return l.size
}

func (l *List) Push(element int) {
	// Create a node
	var tnode *Node = new(Node)
	tnode.data = element
	tnode.next = nil
	l.size++
	if l.head == nil {
		l.head = tnode
		l.tail = tnode
		return
	}
	l.tail.next = tnode
	l.tail = tnode

}

func (l *List) Pop() (int, error) {
    if l.head == nil {
		return -1, errors.New("Pop from empty list")
	}
	if l.head == l.tail {
		tempData := l.head.data
		l.head = nil
		l.tail = nil
		l.size--
		return tempData, nil
	}
	var tNode *Node = l.head
	for tNode.next != l.tail {
		tNode = tNode.next
	}
	tempData := l.tail.data
	tNode.next = nil
	l.tail = tNode
	l.size--
	return tempData, nil
}

func (l *List) Array() []int {
	arr := make([]int, l.Size())
	var tNode *Node = l.head
	i := 0
	for tNode != nil {
		arr[i] = tNode.data
		i++
		tNode = tNode.next
	}
	return arr
}

func (l *List) Reverse() *List {
    if l.head == l.tail {
		return l
	}
	var p *Node = nil
	var m *Node = l.head
	var n *Node = l.head

	for n.next != nil {
		n = n.next
		m.next = p
		p = m
		m = n
	}
	m.next = p
	l.head = m
	return l
}