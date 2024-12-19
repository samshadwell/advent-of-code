package grids

import (
	"bufio"
	"io"
)

type Location struct {
	Row int
	Col int
}

func Up() Location    { return Location{Row: -1, Col: 0} }
func Down() Location  { return Location{Row: 1, Col: 0} }
func Left() Location  { return Location{Row: 0, Col: -1} }
func Right() Location { return Location{Row: 0, Col: 1} }

var cardinalDirections = []Location{
	Right(),
	Down(),
	Left(),
	Up(),
}

func (l Location) Plus(other Location) Location {
	return Location{
		Row: l.Row + other.Row,
		Col: l.Col + other.Col,
	}
}

func (l Location) Minus(other Location) Location {
	return Location{
		Row: l.Row - other.Row,
		Col: l.Col - other.Col,
	}
}

func ParseRuneGrid(reader io.Reader) [][]rune {
	scanner := bufio.NewScanner(reader)
	grid := make([][]rune, 0)
	for scanner.Scan() {
		line := scanner.Text()
		grid = append(grid, []rune(line))
	}
	return grid
}

func FindRune(grid [][]rune, toFind rune) (Location, bool) {
	for rIdx, row := range grid {
		for cIdx, rune := range row {
			if rune == toFind {
				return Location{rIdx, cIdx}, true
			}
		}
	}
	return Location{}, false
}

func EachAdjacent(loc Location, numRows, numCols int, cb func(loc Location)) {
	for _, d := range cardinalDirections {
		new := loc.Plus(d)
		if IsOutOfBounds(new, numRows, numCols) {
			continue
		}
		cb(new)
	}
}

func IsOutOfBounds(loc Location, numRows, numCols int) bool {
	return loc.Row < 0 || loc.Col < 0 || loc.Row >= numRows || loc.Col >= numCols
}
