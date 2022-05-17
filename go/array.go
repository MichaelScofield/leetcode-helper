package main

import (
	"fmt"
	"strconv"
	"strings"
)

func CreateTwoDimensionalArray(s string) [][]int {
	if s[0] != '[' && s[len(s)-1] != ']' {
		panic("invalid input: " + s)
	}
	s = s[1 : len(s)-1]

	var arr [][]int
	var curr []int
	ss := strings.Split(s, ",")
	for i := 0; i < len(ss); i++ {
		token := ss[i]
		if token[0] == '[' {
			curr = []int{ParseInt(token[1:])}
		} else {
			if token[len(token)-1] == ']' {
				curr = append(curr, ParseInt(token[:len(token)-1]))
				arr = append(arr, curr)
			} else {
				curr = append(curr, ParseInt(token))
			}
		}
	}
	return arr
}

func ParseInt(s string) int {
	if i, err := strconv.Atoi(s); err != nil {
		panic(fmt.Sprintf("unable to parse string \"%s\" to integer, err = %s", s, err))
	} else {
		return i
	}
}
