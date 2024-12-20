package main

import (
	"advent-of-code/util/grids"
	"errors"
	"fmt"
	"io"
	"log"
	"os"
	"runtime/pprof"
)

type input struct {
	track      [][]rune
	start, end grids.Location
}

func main() {
	// CPU Profile
	cpuFile, err := os.Create("cpu.prof")
	if err != nil {
		log.Fatal(err)
	}
	defer cpuFile.Close()
	if err := pprof.StartCPUProfile(cpuFile); err != nil {
		log.Fatal(err)
	}
	defer pprof.StopCPUProfile()

	// Memory Profile
	memFile, err := os.Create("mem.prof")
	if err != nil {
		log.Fatal(err)
	}
	defer memFile.Close()

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

	part1 := countGoodCheats(in.track, ordering, 2, 100)
	part2 := countGoodCheats(in.track, ordering, 20, 100)

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)

	// Write memory profile at the end
	if err := pprof.WriteHeapProfile(memFile); err != nil {
		log.Fatal(err)
	}
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
			if in.track[loc.Row][loc.Col] != '#' && !explored {
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
	numRows := len(track)
	numCols := len(track[0])

	memo := make(map[locN]*treeNode, cheatSteps*numRows*numCols)
	for start := range ordering {
		accessible := locationsNStepsAway(track, start, cheatSteps, memo)
		accessible.visitValues(func(end grids.Location) {
			timeSaved := ordering[end] - ordering[start] - distance(start, end)
			if timeSaved >= threshold {
				count++
			}
		})
	}

	return count
}

// Used as key in our memoization of results
type locN struct {
	loc grids.Location
	n   int
}

type treeNode struct {
	hasValue bool
	value    grids.Location
	children []*treeNode
}

func (tn *treeNode) visitValues(f func(grids.Location)) {
	visited := make(map[*treeNode]bool)
	tn.visitValuesHelper(visited, f)
}

func (tn *treeNode) visitValuesHelper(visited map[*treeNode]bool, f func(grids.Location)) {
	if visited[tn] {
		return
	}
	visited[tn] = true
	if tn.hasValue {
		f(tn.value)
	}
	for _, c := range tn.children {
		c.visitValuesHelper(visited, f)
	}
}

func locationsNStepsAway(track [][]rune, start grids.Location, n int, memo map[locN]*treeNode) *treeNode {
	key := locN{start, n}
	if res, ok := memo[key]; ok {
		return res
	}

	res := &treeNode{}
	memo[key] = res

	if n == 0 {
		if track[start.Row][start.Col] != '#' {
			res.hasValue = true
			res.value = start
		}
		return res
	}

	children := make([]*treeNode, 0, 5)
	children = append(children, locationsNStepsAway(track, start, n-1, memo))
	grids.EachAdjacent(start, len(track), len(track[0]), func(loc grids.Location) {
		children = append(children, locationsNStepsAway(track, loc, n-1, memo))
	})

	res.children = children
	return res
}

func distance(start, end grids.Location) int {
	deltaR := start.Row - end.Row
	if deltaR < 0 {
		deltaR = -deltaR
	}

	deltaC := start.Col - end.Col
	if deltaC < 0 {
		deltaC = -deltaC
	}

	return deltaR + deltaC
}
