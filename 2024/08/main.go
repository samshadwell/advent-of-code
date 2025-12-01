package main

import (
	"bufio"
	"fmt"
	"log"
	"math/big"
	"os"
	"slices"

	"advent-of-code/util/grids"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("failed to open file: %v", err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	antennas := make(map[rune][]grids.Location)
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
				antennas[r] = make([]grids.Location, 0, 10)
			}
			antennas[r] = append(antennas[r], grids.Location{Row: row, Col: col})
		}
		row++
	}

	numRows := row

	part1 := 0
	for r := 0; r < numRows; r++ {
		for c := 0; c < numCols; c++ {
			if IsPart1Antinode(grids.Location{Row: r, Col: c}, antennas) {
				part1++
			}
		}
	}

	part2Locations := make(map[grids.Location]bool)
	for _, as := range antennas {
		// For each pair of antennas (a1, a2), add all the locations along the vector pointing from a1 -> a2
		for _, a1 := range as {
			for _, a2 := range as {
				if a1 == a2 {
					continue
				}
				d := a1.Minus(a2)
				slope := SimplifyDisplacement(d)
				part2Locations[a1] = true
				for n := 1; ; n++ {
					res := grids.Location{
						Row: n*slope.Row + a1.Row,
						Col: n*slope.Col + a1.Col,
					}
					if res.Row < 0 || res.Col < 0 || res.Row >= numRows || res.Col >= numCols {
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

func IsPart1Antinode(loc grids.Location, antennas map[rune][]grids.Location) bool {
	for _, locations := range antennas {
		for _, a1 := range locations {
			displacement := a1.Minus(loc)
			if displacement.Row == 0 && displacement.Col == 0 {
				continue
			}
			a2 := loc.Plus(displacement).Plus(displacement)
			if slices.Contains(locations, a2) {
				return true
			}
		}
	}
	return false
}

func SimplifyDisplacement(d grids.Location) grids.Location {
	r := big.NewInt(int64(d.Row))
	c := big.NewInt(int64(d.Col))

	var z, newR, newC big.Int
	z.GCD(nil, nil, r, c)

	newR.Div(r, &z)
	newC.Div(c, &z)
	return grids.Location{
		Row: int(newR.Int64()),
		Col: int(newC.Int64()),
	}
}
