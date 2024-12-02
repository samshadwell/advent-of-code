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
		// Exploiting the fact that zero-values are returned if not present
		prevCount := rightCounts[second]
		rightCounts[second] = prevCount + 1
	}

	sort.Ints(left)

	totalSimilarity := 0
	for _, l := range left {
		similarity := l * rightCounts[l]
		totalSimilarity += similarity
	}

	fmt.Printf("Total similarity: %d\n", totalSimilarity)
}
