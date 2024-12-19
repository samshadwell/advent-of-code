package main

import (
	"bufio"
	"fmt"
	"log"
	"os"

	"advent-of-code/util/grids"
)

type RegionStats struct {
	area       int
	perimeter  int
	numCorners int
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening file: %v", err)
	}
	defer file.Close()

	board := make([][]rune, 0, 1)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		board = append(board, []rune(line))
	}
	if len(board) == 0 {
		log.Fatalf("did not find any lines in input, expected at least one")
	}

	part1, part2 := ComputeCosts(board)
	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}

func ComputeCosts(board [][]rune) (int, int) {
	if len(board) == 0 {
		return 0, 0
	}

	part1 := 0
	part2 := 0
	visited := make(map[grids.Location]bool)

	for row := range board {
		for col := range board[row] {
			loc := grids.Location{Row: row, Col: col}
			stats, err := ExploreRegion(board, loc, visited)
			if err != nil {
				log.Fatalf("error while exploring region: %v", err)
			}
			part1 += stats.area * stats.perimeter
			part2 += stats.area * stats.numCorners
		}
	}

	return part1, part2
}

var baseDirections = []grids.Location{
	{Row: -1, Col: 0},
	{Row: 1, Col: 0},
	{Row: 0, Col: -1},
	{Row: 0, Col: 1},
}

var allDirections = []grids.Location{
	{Row: -1, Col: -1}, // Upper left
	{Row: -1, Col: 0},  // Above
	{Row: -1, Col: 1},  // Upper right
	{Row: 0, Col: 1},   // Right
	{Row: 1, Col: 1},   // Lower right
	{Row: 1, Col: 0},   // Below
	{Row: 1, Col: -1},  // Lower left
	{Row: 0, Col: -1},  // Left
}

// Indexes in above array that represent the four possible "convex" type corners, as in:
//
//	AA
//	AX
//
// from the perspective of the "A" at upper-left. Convex will be F,T,F in differences array
// at the given indeces if such a corner exists
var convexCorners = [][]int{
	{7, 0, 1},
	{1, 2, 3},
	{3, 4, 5},
	{5, 6, 7},
}

// Indexes in above array that represent the four possible "concave" type corners, as in:
//
//	AX
//	XX
//
// from the perspective of the "A" at upper-left. Concave will be T,T in the differences array
// at the given indeces if such a corner exists.
var concaveCorners = [][]int{
	{7, 1},
	{1, 3},
	{3, 5},
	{5, 7},
}

func ExploreRegion(board [][]rune, start grids.Location, visited map[grids.Location]bool) (RegionStats, error) {
	numRows := len(board)
	numCols := len(board[0])
	if grids.IsOutOfBounds(start, numRows, numCols) {
		return RegionStats{}, fmt.Errorf("cannot start exploration off of board at (%d,%d)", start.Row, start.Col)
	}

	if visited[start] {
		// If visited, stats for this have already been counted
		return RegionStats{0, 0, 0}, nil
	}

	visited[start] = true
	label := board[start.Row][start.Col]

	area := 1
	perimeter := 0
	numCorners := 0

	for _, d := range baseDirections {
		next := start.Plus(d)
		if grids.IsOutOfBounds(next, numRows, numCols) || board[next.Row][next.Col] != label {
			perimeter += 1
		} else {
			explored, err := ExploreRegion(board, next, visited)
			if err != nil {
				return RegionStats{}, err
			}
			area += explored.area
			perimeter += explored.perimeter
			numCorners += explored.numCorners
		}
	}

	differences := make([]bool, len(allDirections))
	for i, d := range allDirections {
		newLoc := start.Plus(d)
		differences[i] = label != GetOrDefault(board, newLoc, '.')
	}

	for _, c := range convexCorners {
		if !differences[c[0]] && differences[c[1]] && !differences[c[2]] {
			numCorners++
		}
	}
	for _, c := range concaveCorners {
		if differences[c[0]] && differences[c[1]] {
			numCorners++
		}
	}

	return RegionStats{area: area, perimeter: perimeter, numCorners: numCorners}, nil
}

func GetOrDefault(board [][]rune, loc grids.Location, r rune) rune {
	if grids.IsOutOfBounds(loc, len(board), len(board[0])) {
		return r
	}
	return board[loc.Row][loc.Col]
}
