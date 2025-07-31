package binarysearchtree

type BinarySearchTree struct {
	left  *BinarySearchTree
	data  int
	right *BinarySearchTree
}

// NewBst creates and returns a new BinarySearchTree.
func NewBst(i int) *BinarySearchTree {
	var tempBST *BinarySearchTree = new(BinarySearchTree)
	tempBST.data = i
	tempBST.left = nil
	tempBST.right = nil
	return tempBST
}

// Insert inserts an int into the BinarySearchTree.
// Inserts happen based on the rules of a binary search tree
func (bst *BinarySearchTree) Insert(i int) {
	var trav1 *BinarySearchTree = bst
	var trav2 *BinarySearchTree = nil
	if trav1 == nil {
		return
	}
	cond := 0
	for trav1 != nil {
		if i <= trav1.data {
			trav2 = trav1
			trav1 = trav1.left
			cond = -1
		} else {
			trav2 = trav1
			trav1 = trav1.right
			cond = 1
		}
	}
	var temp *BinarySearchTree = NewBst(i)
	if cond == -1 {
		trav2.left = temp
	} else {
		trav2.right = temp
	}
}

// SortedData returns the ordered contents of BinarySearchTree as an []int.
// The values are in increasing order starting with the lowest int value.
// A BinarySearchTree that has the numbers [1,3,7,5] added will return the
// []int [1,3,5,7].
func (bst *BinarySearchTree) SortedData() []int {
	nums := []int{}
	var Inorder func(b *BinarySearchTree)
	Inorder = func(b *BinarySearchTree) {
		if b != nil {
			Inorder(b.left)
			nums = append(nums, b.data)
			Inorder(b.right)
		}
	}
	Inorder(bst)
	return nums
}