package main

import (
	"bufio"
	"fmt"
	"log"
	"math/big"
	"os"
	"slices"
)

type Location struct {
	row int
	col int
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("failed to open file: %v", err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	antennas := make(map[rune][]Location)
	row := 0
	numCols := -1
	for scanner.Scan() {
		line := scanner.Text()

		if numCols == -1 {
			numCols = len(line)
		} else if numCols != len(line) {
			log.Fatalf("found inconsistent number of columns in rows of map. Want %d, got %d in row %d", numCols, len(line), row)
		}

		for col, r := range line {
			if r == '.' {
				continue
			}

			if _, ok := antennas[r]; !ok {
				antennas[r] = make([]Location, 0, 10)
			}
			antennas[r] = append(antennas[r], Location{row, col})
		}
		row++
	}

	numRows := row

	part1 := 0
	for r := 0; r < numRows; r++ {
		for c := 0; c < numCols; c++ {
			if IsPart1Antinode(Location{r, c}, antennas) {
				part1++
			}
		}
	}

	part2Locations := make(map[Location]bool)
	for _, as := range antennas {
		// For each pair of antennas (a1, a2), add all the locations along the vector pointing from a1 -> a2
		for _, a1 := range as {
			for _, a2 := range as {
				if a1 == a2 {
					continue
				}
				slope := SimplifyDisplacement(Location{a1.row - a2.row, a1.col - a2.col})
				part2Locations[a1] = true
				for n := 1; ; n++ {
					res := Location{
						n*slope.row + a1.row,
						n*slope.col + a1.col,
					}
					if res.row < 0 || res.col < 0 || res.row >= numRows || res.col >= numCols {
						break
					}
					part2Locations[res] = true
				}
			}
		}
	}

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", len(part2Locations))
}

func IsPart1Antinode(loc Location, antennas map[rune][]Location) bool {
	for _, locations := range antennas {
		for _, a1 := range locations {
			displacement := Location{a1.row - loc.row, a1.col - loc.col}
			if displacement.row == 0 && displacement.col == 0 {
				continue
			}
			a2 := Location{
				loc.row + 2*displacement.row,
				loc.col + 2*displacement.col,
			}
			if slices.Contains(locations, a2) {
				return true
			}
		}
	}
	return false
}

func SimplifyDisplacement(d Location) Location {
	r := big.NewInt(int64(d.row))
	c := big.NewInt(int64(d.col))

	var z, newR, newC big.Int
	z.GCD(nil, nil, r, c)

	newR.Div(r, &z)
	newC.Div(c, &z)
	return Location{
		row: int(newR.Int64()),
		col: int(newC.Int64()),
	}
}
