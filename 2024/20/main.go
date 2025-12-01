package main

import (
	"advent-of-code/util/grids"
	"advent-of-code/util/mymath"
	"errors"
	"fmt"
	"io"
	"log"
	"os"
)

const (
	part1NumCheats = 2
	part2NumCheats = 20
	scoreThreshold = 100
)

type input struct {
	track      [][]rune
	start, end grids.Location
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening input file: %v", err)
	}
	in, err := parseInput(file)
	if err != nil {
		log.Fatalf("error while parsing input: %v", err)
	}

	ordering, err := getOrdering(in)
	if err != nil {
		log.Fatalf("error while traversing route from start to end: %v", err)
	}

	part1 := countGoodCheats(in.track, ordering, part1NumCheats, scoreThreshold)
	part2 := countGoodCheats(in.track, ordering, part2NumCheats, scoreThreshold)

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}

func parseInput(r io.Reader) (input, error) {
	track := grids.ParseRuneGrid(r)
	start, ok := grids.FindRune(track, 'S')
	if !ok {
		return input{}, errors.New("did not find start position on track")
	}

	end, ok := grids.FindRune(track, 'E')
	if !ok {
		return input{}, errors.New("did not find end position on track")
	}

	return input{track, start, end}, nil
}

func getOrdering(in input) (map[grids.Location]int, error) {
	ordering := make(map[grids.Location]int)
	ordering[in.start] = 0

	current := in.start
	n := 1
	for current != in.end {
		found := false
		var next grids.Location
		grids.EachAdjacent(current, len(in.track), len(in.track[0]), func(loc grids.Location) {
			_, explored := ordering[loc]
			if grids.GetOrDefault(in.track, loc, '#') != '#' && !explored {
				next = loc
				found = true
				return
			}
		})
		if !found {
			return nil, errors.New("did not find path from start to end")
		}

		ordering[next] = n
		current = next
		n++
	}

	return ordering, nil
}

func countGoodCheats(track [][]rune, ordering map[grids.Location]int, cheatSteps, threshold int) int {
	count := 0
	for start, startOrd := range ordering {
		for stepsUp := -cheatSteps; stepsUp <= cheatSteps; stepsUp++ {
			stepsRemaining := cheatSteps - mymath.IntAbs(stepsUp)

			for stepsRight := -stepsRemaining; stepsRight <= stepsRemaining; stepsRight++ {
				dX := grids.Right().ScalarMult(stepsRight)
				dY := grids.Up().ScalarMult(stepsUp)

				end := start.Plus(dX).Plus(dY)
				if grids.GetOrDefault(track, end, '#') == '#' {
					continue
				}

				timeSaved := ordering[end] - startOrd - distance(start, end)
				if timeSaved >= threshold {
					count++
				}
			}
		}
	}

	return count
}

func distance(start, end grids.Location) int {
	deltaR := start.Row - end.Row
	deltaC := start.Col - end.Col

	return mymath.IntAbs(deltaR) + mymath.IntAbs(deltaC)
}
