package main

import (
	"advent-of-code/util/grids"
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

var start = grids.Location{Col: 0, Row: 0}
var goal = grids.Location{Col: 70, Row: 70}
var coordinateRegexp = regexp.MustCompile(`(\d+),(\d+)`)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening input file: %v", err)
	}

	input := make([]grids.Location, 0)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		match := coordinateRegexp.FindStringSubmatch(line)
		if match == nil || len(match) != 3 {
			log.Fatalf("error parsing input line %s, expected regex with exactly 3 matching parts, got: %v", line, match)
		}
		c, err := strconv.Atoi(match[1])
		if err != nil {
			log.Fatalf("error converting coordinate X of %s to int: %v", line, err)
		}
		r, err := strconv.Atoi(match[2])
		if err != nil {
			log.Fatalf("error converting coordinate Y of %s to int: %v", line, err)
		}

		input = append(input, grids.Location{Col: c, Row: r})
	}

	path := bfs(input[:1024], start, goal)
	fmt.Printf("Part 1: %d\n", len(path)-1)

	lastIdx := 1024
	for ; lastIdx < len(input); lastIdx++ {
		if _, ok := path[input[lastIdx]]; !ok {
			// If the fallen byte isn't on the current escape route anyway, don't re-run BFS
			continue
		}

		// Implicit else: previous escape route is now blocked, try a different route
		path = bfs(input[:lastIdx+1], start, goal)
		if path == nil {
			break
		}
	}
	lastByte := input[lastIdx]
	fmt.Printf("Part 2: %d,%d\n", lastByte.Col, lastByte.Row)
}

// Returns the set of nodes along one of the shortest paths from start to goal, or nil if no path exists
func bfs(corrupt []grids.Location, start, goal grids.Location) map[grids.Location]bool {
	corrupted := make(map[grids.Location]bool, len(corrupt))
	for _, loc := range corrupt {
		corrupted[loc] = true
	}

	parents := make(map[grids.Location]*grids.Location)
	parents[start] = nil

	queue := make([]grids.Location, 0)
	queue = append(queue, start)
	for len(queue) > 0 {
		var loc grids.Location
		loc, queue = queue[0], queue[1:]
		grids.EachAdjacent(loc, goal.Row+1, goal.Col+1, func(l grids.Location) {
			if _, ok := parents[l]; corrupted[l] || ok {
				return
			}
			parents[l] = &loc
			queue = append(queue, l)
		})
	}

	if _, ok := parents[goal]; !ok {
		return nil
	}

	path := make(map[grids.Location]bool, 0)
	for l := &goal; l != nil; l = parents[*l] {
		path[*l] = true
	}

	return path
}
