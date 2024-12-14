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

const maxHeight = 9

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening file: %v", err)
	}
	defer file.Close()

	topo := make([][]int, 0, 1)
	zeroes := make(map[Location]bool)

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
				zeroes[Location{row: row, col: col}] = true
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

var offsets = []Location{
	{-1, 0},
	{1, 0},
	{0, -1},
	{0, 1},
}

func ReachableSummits(topo [][]int, loc Location) map[Location]bool {
	if len(topo) < 1 {
		log.Fatal("cannot find trails on empty map")
	}

	if topo[loc.row][loc.col] == maxHeight {
		return map[Location]bool{{loc.row, loc.col}: true}
	}

	summits := make(map[Location]bool)
	for _, offset := range offsets {
		newR := loc.row + offset.row
		newC := loc.col + offset.col
		if newR < 0 || newR >= len(topo) || newC < 0 || newC >= len(topo[0]) {
			continue
		} else if topo[newR][newC] != 1+topo[loc.row][loc.col] {
			continue
		}

		rs := ReachableSummits(topo, Location{newR, newC})
		for s := range rs {
			summits[s] = true
		}
	}

	return summits
}

func DistinctPaths(topo [][]int, loc Location) int {
	if len(topo) < 1 {
		log.Fatal("cannot find trails on empty map")
	}

	if topo[loc.row][loc.col] == maxHeight {
		return 1
	}

	total := 0
	for _, offset := range offsets {
		newR := loc.row + offset.row
		newC := loc.col + offset.col
		if newR < 0 || newR >= len(topo) || newC < 0 || newC >= len(topo[0]) {
			continue
		} else if topo[newR][newC] != 1+topo[loc.row][loc.col] {
			continue
		}

		total += DistinctPaths(topo, Location{newR, newC})
	}

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
