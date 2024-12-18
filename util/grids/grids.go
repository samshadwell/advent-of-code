package grids

import (
	"bufio"
	"os"
)

type Location struct {
	Row int
	Col int
}

func (l Location) Plus(other Location) Location {
	return Location{
		Row: l.Row + other.Row,
		Col: l.Col + other.Col,
	}
}

func ParseRuneGrid(file *os.File) [][]rune {
	scanner := bufio.NewScanner(file)
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

func IsOutOfBounds(grid [][]rune, loc Location) bool {
	return loc.Row < 0 || loc.Col < 0 || loc.Row >= len(grid) || loc.Col >= len(grid[loc.Row])
}
