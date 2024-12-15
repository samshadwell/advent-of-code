package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Location struct {
	row int
	col int
}

func (l *Location) Plus(other Location) Location {
	return Location{
		row: l.row + other.row,
		col: l.col + other.col,
	}
}

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

	numRows := len(board)
	numCols := len(board[0])
	visited := make(map[Location]bool, numRows*numCols)

	part1Cost := 0
	part2Cost := 0
	for r := 0; r < numRows; r++ {
		for c := 0; c < numCols; c++ {
			l := Location{r, c}
			stats, err := ExploreRegion(board, l, visited)
			if err != nil {
				log.Fatalf("error while exploring region: %v", err)
			}
			part1Cost += stats.area * stats.perimeter
			part2Cost += stats.area * stats.numCorners
		}
	}

	return part1Cost, part2Cost
}

var baseDirections = []Location{
	{-1, 0},
	{1, 0},
	{0, -1},
	{0, 1},
}

var allDirections = []Location{
	{-1, -1}, // Upper left
	{-1, 0},  // Above
	{-1, 1},  // Upper right
	{0, 1},   // Right
	{1, 1},   // Lower right
	{1, 0},   // Below
	{1, -1},  // Lower left
	{0, -1},  // Left
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

func ExploreRegion(board [][]rune, start Location, visited map[Location]bool) (RegionStats, error) {
	if OutOfBounds(board, start) {
		return RegionStats{}, fmt.Errorf("cannot start exploration off of board at (%d,%d)", start.row, start.col)
	}

	if visited[start] {
		// If visited, stats for this have already been counted
		return RegionStats{0, 0, 0}, nil
	}

	visited[start] = true
	label := board[start.row][start.col]

	area := 1
	perimeter := 0
	numCorners := 0

	for _, d := range baseDirections {
		next := start.Plus(d)
		if OutOfBounds(board, next) || board[next.row][next.col] != label {
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

func OutOfBounds(board [][]rune, loc Location) bool {
	return loc.row < 0 || loc.col < 0 || loc.row >= len(board) || loc.col >= len(board[loc.row])
}

func GetOrDefault(board [][]rune, loc Location, r rune) rune {
	if OutOfBounds(board, loc) {
		return r
	}
	return board[loc.row][loc.col]
}
