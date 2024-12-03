package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var expressionRegexp = regexp.MustCompile(`mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)`)

func main() {
	bytes, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatalf("failed to read file: %v", err)
	}

	content := string(bytes)
	matches := expressionRegexp.FindAllStringSubmatch(content, -1)
	if matches == nil {
		log.Fatalf("found 0 matches in file, expected at least 1")
	}
	part1 := 0
	part2 := 0
	enabled := true
	for _, match := range matches {
		if isMultiplyExpr(match) {
			l, err := strconv.Atoi(match[1])
			if err != nil {
				log.Fatalf("error converting left operand bytes to number: %s", match[1])
			}

			r, err := strconv.Atoi(match[2])
			if err != nil {
				log.Fatalf("error converting right operand bytes to number: %s", match[2])
			}
			part1 += l * r
			if enabled {
				part2 += l * r
			}
		} else if isDoExpr(match) {
			enabled = true
		} else if isDontExpr(match) {
			enabled = false
		} else {
			log.Fatalf("programmer error, should be unreachable statement, match: %v", match)
		}
	}

	fmt.Printf("part 1: %d\n", part1)
	fmt.Printf("part 2: %d\n", part2)
}

func isMultiplyExpr(match []string) bool {
	return len(match) == 3 && strings.HasPrefix(match[0], "mul")
}

func isDoExpr(match []string) bool {
	return len(match) > 0 && match[0] == "do()"
}

func isDontExpr(match []string) bool {
	return len(match) > 0 && match[0] == "don't()"
}
