package main

import (
	"bufio"
	"fmt"
	"log"
	"os"

	"advent-of-code/util/grids"
)

const maxHeight = 9

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening file: %v", err)
	}
	defer file.Close()

	topo := make([][]int, 0, 1)
	zeroes := make(map[grids.Location]bool)

	scanner := bufio.NewScanner(file)
	row := 0
	for scanner.Scan() {
		line := scanner.Bytes()
		topo = append(topo, make([]int, len(line)))
		for col, b := range line {
			height, err := DigitToInt(b)
			if err != nil {
				log.Fatalf("error while parsing height: %v", err)
			}

			topo[row][col] = height
			if height == 0 {
				zeroes[grids.Location{Row: row, Col: col}] = true
			}
		}
		row++
	}

	part1 := 0
	part2 := 0
	for loc := range zeroes {
		part1 += len(ReachableSummits(topo, loc))
		part2 += DistinctPaths(topo, loc)
	}

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}

func ReachableSummits(topo [][]int, loc grids.Location) map[grids.Location]bool {
	if len(topo) < 1 {
		log.Fatal("cannot find trails on empty map")
	}

	if topo[loc.Row][loc.Col] == maxHeight {
		return map[grids.Location]bool{{Row: loc.Row, Col: loc.Col}: true}
	}

	summits := make(map[grids.Location]bool)
	grids.EachAdjacent(loc, len(topo), len(topo[0]), func(newL grids.Location) {
		if topo[newL.Row][newL.Col] != 1+topo[loc.Row][loc.Col] {
			return
		}

		rs := ReachableSummits(topo, newL)
		for s := range rs {
			summits[s] = true
		}
	})

	return summits
}

func DistinctPaths(topo [][]int, loc grids.Location) int {
	if len(topo) < 1 {
		log.Fatal("cannot find trails on empty map")
	}

	if topo[loc.Row][loc.Col] == maxHeight {
		return 1
	}

	total := 0
	grids.EachAdjacent(loc, len(topo), len(topo[0]), func(newL grids.Location) {
		if topo[newL.Row][newL.Col] != 1+topo[loc.Row][loc.Col] {
			return
		}

		total += DistinctPaths(topo, newL)
	})

	return total
}

const asciiZero = 48
const asciiNine = 57

func DigitToInt(b byte) (int, error) {
	if b < asciiZero || b > asciiNine {
		return 0, fmt.Errorf("cannot convert non-digit byte to int: %x", b)
	}
	return int(b - asciiZero), nil
}
