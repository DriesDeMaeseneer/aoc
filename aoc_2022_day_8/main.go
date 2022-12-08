package main

import (
	"fmt"
	"os"
	"strings"
)

func parse() []string {
	content, err := os.ReadFile("day_8.input")
	if err != nil {
		return nil
	}
	return strings.Split(string(content), "\n")
}

func is_visible(test []string, i int, j int) bool {

	var currentChar = test[i][j]

	var visLeft = false
	var visRight = false
	var left string
	var right string

	if j == 0 {
		return true
	} else {
		left = test[i][0:j]
		visLeft = true
	}
	if j == len(test[0])-1 {
		return true
	} else {
		right = test[i][j+1 : len(test[0])]
		visRight = true
	}
	for k := 0; k < len(left); k++ {
		if left[k] >= currentChar {
			visLeft = false
			break
		}
	}
	if visLeft {
		return true
	}
	for k := 0; k < len(right); k++ {
		if right[k] >= currentChar {
			visRight = false
			break
		}
	}
	if visRight {
		return true
	}
	var visTop = true
	var visBottom = true

	if i == 0 {
		visTop = false
		return true
	}
	for k := 0; k < i; k++ {
		if test[k][j] >= currentChar {
			visTop = false
			break
		}

	}
	if visTop {
		return true
	}
	if i == len(test)-1 {
		visBottom = false
		return true
	}
	for k := 1; k < len(test)-1-i; k++ {
		if test[i+k][j] >= currentChar {
			visBottom = false
			break
		}
	}
	return visBottom
}
func view_distance(test []string, i int, j int) int {
	var currentChar = test[i][j]
	var left_dist = j
	var right_dist = len(test[i]) - j - 1
	var top_dist int = i
	var bottom_dist int = len(test) - 2- i

	var left string
	var right string

	if j == 0 {
		return 0
	} else {
		left = test[i][0:j]
	}
	if j == len(test[i])-1 {
		return 0
	} else {
		right = test[i][j+1 : len(test[0])]
	}
	for k := 0; k < j; k++ {
		if left[k] >= currentChar {
			left_dist = j - k
		}
	}
	for k := 0; k < len(right); k++ {
		if right[k] >= currentChar {
			right_dist = k + 1
			break
		}
	}

	if i == 0 {
		return 0
	}
	for k := 0; k < i; k++ {
		if test[k][j] >= currentChar {
			top_dist = i - k
		}

	}
	if i == len(test)-1 {
		return 0
	}
	for k := 1; k < len(test)-1-i; k++ {
		if test[i+k][j] >= currentChar {
			bottom_dist = k 
			break
		}
	}
	// fmt.Println(i, j, left_dist, right_dist, top_dist, bottom_dist)
	return left_dist * right_dist * top_dist * bottom_dist

}
func max_view_distance(test []string) int {
	var TotalVisible = 0


	for i := 1; i < len(test) - 2; i++ {
		for j := 1; j < len(test[i]) - 1; j++ {
			dist := view_distance(test, i, j)
			if dist > TotalVisible {
				TotalVisible = dist
			}
		}
	}
	return TotalVisible
}

func total_visible(test []string) int {

	var TotalVisible = 0

	for i := 0; i < len(test); i++ {
		for j := 0; j < len(test[i]); j++ {
			if is_visible(test, i, j) {
				TotalVisible += 1
			}
		}
	}
	return TotalVisible
}

func main() {
	var test = parse()
	var total = total_visible(test)
	fmt.Println(total)
	total = max_view_distance(test)
	fmt.Println(total)

}
