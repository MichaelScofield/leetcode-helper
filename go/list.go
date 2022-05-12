package main

import (
	"fmt"
	"strconv"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func NewList(vals []int) *ListNode {
	if len(vals) == 0 {
		return nil
	}

	head := ListNode{Val: vals[0], Next: nil}
	curr := &head
	for i := 1; i < len(vals); i++ {
		node := ListNode{Val: vals[i], Next: nil}
		curr.Next = &node
		curr = curr.Next
	}
	return &head
}

func PrintList(head *ListNode) {
	s := "["

	node := head
	for node.Next != nil {
		s += fmt.Sprintf("%d, ", node.Val)
		node = node.Next
	}
	s += strconv.Itoa(node.Val)

	s += "]"
	fmt.Println(s)
}
