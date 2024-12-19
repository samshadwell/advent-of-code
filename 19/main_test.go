package main

import (
	"strings"
	"testing"
)

func TestExample(t *testing.T) {
	exampleInput := `r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
`
	input, err := parseInput(strings.NewReader(exampleInput))
	if err != nil {
		t.Fatalf("expected parseInput to return nil error, but got: %v", err)
	}

	part1, part2 := countPossible(input)
	if part1 != 6 {
		t.Fatalf("countPossible did not return correct answer for part 1, want: 6, got: %d", part1)
	}

	if part2 != 16 {
		t.Fatalf("countPossible did not return correct answer for part 2, want: 16, got: %d", part2)
	}
}
