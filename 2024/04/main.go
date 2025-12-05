package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

var part1Word = []rune{
	'X', 'M', 'A', 'S'}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("failed to open file: %v", err)
	}
	defer func() { _ = file.Close() }()

	scanner := bufio.NewScanner(file)
	grid := make([][]rune, 0, 10)
	for scanner.Scan() {
		grid = append(grid, []rune(scanner.Text()))
	}

	part1 := 0
	part2 := 0
	for xIdx, line := range grid {
		for yIdx := range line {
			part1 += SearchAround(grid, xIdx, yIdx)
			if IsXMas(grid, xIdx, yIdx) {
				part2 += 1
			}
		}
	}

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}

// Searches for "XMAS" originating at the given coordinate, in any direction
func SearchAround(grid [][]rune, startX int, startY int) int {
	found := 0
	for xMod := -1; xMod <= 1; xMod++ {
		for yMod := -1; yMod <= 1; yMod++ {
			if xMod == 0 && yMod == 0 {
				continue
			}

			for step := 0; step < len(part1Word); step++ {
				x := startX + step*xMod
				y := startY + step*yMod
				if x < 0 || x >= len(grid) || y < 0 || y >= len(grid[0]) {
					break
				}

				if grid[x][y] != part1Word[step] {
					break
				}

				if step == len(part1Word)-1 {
					found += 1
				}
			}
		}
	}

	return found
}

// Searches for an X of "MAS" around a center coordinate (the "A")
func IsXMas(grid [][]rune, startX, startY int) bool {
	if startX < 1 || startX > len(grid)-2 || startY < 1 || startY > len(grid[0])-2 {
		return false
	}

	diags := [][]rune{
		{grid[startX-1][startY-1], grid[startX][startY], grid[startX+1][startY+1]},
		{grid[startX+1][startY-1], grid[startX][startY], grid[startX-1][startY+1]},
	}

	diag1 := string(diags[0])
	diag2 := string(diags[1])

	return (diag1 == "MAS" || diag1 == "SAM") && (diag2 == "MAS" || diag2 == "SAM")
}
