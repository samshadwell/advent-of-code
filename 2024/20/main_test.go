package main

import (
	"fmt"
	"strings"
	"testing"
)

var part1Tests = []struct{ cheatSteps, threshold, want int }{
	{2, 64, 1},
	{2, 41, 1},
	{2, 40, 2},
	{2, 2, 44},
	{20, 76, 3},
	{20, 75, 3},
	{20, 74, 7},
	{20, 73, 7},
	{20, 72, 29},
}

const exampleTrack = `###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############`

func TestPart1(t *testing.T) {
	in, err := parseInput(strings.NewReader(exampleTrack))
	if err != nil {
		t.Fatalf("expected parseInput to return nil error, but got: %v", err)
	}

	ordering, err := getOrdering(in)
	if err != nil {
		t.Fatalf("expected getOrdering to return nil error, but got: %v", err)
	}

	for _, tc := range part1Tests {
		name := fmt.Sprintf("cheatSteps: %d, threshold %d", tc.cheatSteps, tc.threshold)
		t.Run(name, func(t *testing.T) {
			got := countGoodCheats(in.track, ordering, tc.cheatSteps, tc.threshold)
			if got != tc.want {
				t.Fatalf("countGoodCheats did not return expected value. want: %d, got: %d", tc.want, got)
			}
		})
	}
}
