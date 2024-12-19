package main

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"log"
	"os"
	"strings"
)

type input struct {
	patterns      []string
	targetDesigns []string
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening input file: %v", err)
	}
	defer file.Close()

	input, err := parseInput(file)
	if err != nil {
		log.Fatalf("error while parsing input: %v", err)
	}

	part1, part2 := countPossible(input)

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}

func parseInput(r io.Reader) (input, error) {
	scanner := bufio.NewScanner(r)
	ok := scanner.Scan()
	if !ok {
		return input{}, errors.New("input malformed, expected at least one line, got none")
	}
	p := scanner.Text()
	patterns := strings.Split(p, ", ")

	ok = scanner.Scan()
	if !ok {
		return input{}, errors.New("input malformed, expected at least two lines, got one")
	}
	empty := scanner.Text()
	if empty != "" {
		return input{}, fmt.Errorf("input malformed, expected empty line between towels and target patterns, got '%s'", empty)
	}

	targets := make([]string, 0)
	for scanner.Scan() {
		t := scanner.Text()
		targets = append(targets, t)
	}

	return input{patterns: patterns, targetDesigns: targets}, nil
}

func countPossible(in input) (int, int) {
	memo := make(map[string]int)

	possible := 0
	totalWays := 0
	for _, target := range in.targetDesigns {
		nw := numWays(target, in.patterns, memo)
		totalWays += nw
		if nw > 0 {
			possible++
		}
	}
	return possible, totalWays
}

func numWays(target string, components []string, memo map[string]int) int {
	if target == "" {
		return 1
	}
	if res, ok := memo[target]; ok {
		return res
	}

	count := 0
	for _, c := range components {
		if strings.HasPrefix(target, c) {
			count += numWays(strings.TrimPrefix(target, c), components, memo)
		}
	}

	memo[target] = count
	return count
}
