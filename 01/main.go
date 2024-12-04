package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("failed to open file: %v", err)
	}

	scanner := bufio.NewScanner(file)
	left := make([]int, 0)
	right := make([]int, 0)
	rightCounts := make(map[int]int, 0)
	for scanner.Scan() {
		line := scanner.Text()
		split := strings.Fields(line)
		if len(split) != 2 {
			log.Fatalf("expected exactly 2 fields in line, error parsing '%s'", line)
		}

		first, err := strconv.Atoi(split[0])
		if err != nil {
			log.Fatalf("error parsing number %s", split[0])
		}
		left = append(left, first)

		second, err := strconv.Atoi(split[1])
		if err != nil {
			log.Fatalf("error parsing number %s", split[1])
		}
		right = append(right, second)

		// Exploiting the fact that zero-values are returned if not present
		prevCount := rightCounts[second]
		rightCounts[second] = prevCount + 1
	}

	sort.Ints(left)
	sort.Ints(right)

	if len(left) != len(right) {
		log.Fatalf("expected left and right lists to contain equal elements, they did not")
	}

	part1 := 0
	part2 := 0
	for i, l := range left {
		r := right[i]
		distance := l - r
		if distance < 0 {
			distance = -distance
		}
		part1 += distance

		similarity := l * rightCounts[l]
		part2 += similarity
	}

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}
