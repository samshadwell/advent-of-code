package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"os"
	"runtime/pprof"
	"strings"
	"time"

	"github.com/pterm/pterm"
)

type position struct {
	row int
	col int
}

var directions = []struct{ deltaR, deltaC int }{
	{deltaR: -1, deltaC: 0}, // Up
	{deltaR: 0, deltaC: 1},  // Right
	{deltaR: 1, deltaC: 0},  // Down
	{deltaR: 0, deltaC: -1}, // Left
}

var printFlag = flag.Bool("print", false, "Set to print a visualization of walking to console")
var inputFlag = flag.String("file", "input.txt", "File to read as input")
var cpuprofile = flag.String("cpuprofile", "", "write cpu profile to file")
var memprofile = flag.String("memprofile", "", "write memory profile to file")

func main() {
	flag.Parse()
	if *cpuprofile != "" {
		f, err := os.Create(*cpuprofile)
		if err != nil {
			log.Fatal(err)
		}
		err = pprof.StartCPUProfile(f)
		if err != nil {
			log.Fatal("failed to start cpu profile, aborting")
		}
		defer pprof.StopCPUProfile()
	}

	board, start := parseInput()

	var area *pterm.AreaPrinter
	var err error
	if *printFlag {
		area, err = pterm.DefaultArea.WithCenter().Start()
		if err != nil {
			log.Fatal("Failed to start output area")
		}
	}
	baseRun := run(area, board, start)

	part2 := 0
	for pos := range baseRun.squaresVisited {
		board[pos.row][pos.col] = '#'
		loopRun := run(nil, board, start)
		if !loopRun.terminates {
			part2 += 1
		}
		board[pos.row][pos.col] = '.'
	}

	fmt.Printf("Part 1: %d\n", len(baseRun.squaresVisited))
	fmt.Printf("Part 2: %d\n", part2)

	if *memprofile != "" {
		f, err := os.Create(*memprofile)
		if err != nil {
			log.Fatal("could not create memory profile: ", err)
		}
		defer f.Close()
		if err := pprof.WriteHeapProfile(f); err != nil {
			log.Fatal("could not write memory profile: ", err)
		}
	}
}

func parseInput() ([][]rune, position) {
	file, err := os.Open(*inputFlag)
	if err != nil {
		log.Fatalf("failed to open file: %v", err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	board := make([][]rune, 0)
	start := position{-1, -1}

	for scanner.Scan() {
		line := scanner.Text()
		runes := []rune(line)
		for i, c := range runes {
			if c == '^' {
				start = position{len(board), i}
				runes[i] = '.'
			}
		}
		board = append(board, []rune(line))
	}

	if start.row == -1 && start.col == -1 {
		log.Fatalf("Did not find guard position while parsing board")
	}
	if len(board) == 0 {
		log.Fatalf("Did not parse any rows of board")
	}
	for _, r := range board {
		if len(r) != len(board[0]) {
			log.Fatalf("Board is not square")
		}
	}

	return board, start
}

type runResult struct {
	squaresVisited map[position][]visitInfo
	terminates     bool
}

type visitInfo struct {
	direction int
	sequence  int
}

func run(printer *pterm.AreaPrinter, board [][]rune, start position) runResult {
	sequence := 0
	pos := start
	dir := 0
	visited := make(map[position][]visitInfo, len(board))
	for {
		sequence += 1
		printBoard(printer, board, pos, dir, visited)

		if _, ok := visited[pos]; !ok {
			visited[pos] = make([]visitInfo, 0, 4)
		}
		for _, priorVisit := range visited[pos] {
			if priorVisit.direction == dir {
				return runResult{
					squaresVisited: visited,
					terminates:     false,
				}
			}
		}

		visited[pos] = append(visited[pos], visitInfo{
			direction: dir,
			sequence:  sequence,
		})

		newPos := position{
			row: pos.row + directions[dir].deltaR,
			col: pos.col + directions[dir].deltaC,
		}

		if isOffBoard(board, newPos) {
			break
		}

		if board[newPos.row][newPos.col] == '#' {
			dir = nextDir(dir)
		} else {
			pos = newPos
		}
	}

	return runResult{
		squaresVisited: visited,
		terminates:     true,
	}
}

func isOffBoard(board [][]rune, pos position) bool {
	return pos.row < 0 || pos.row >= len(board) ||
		pos.col < 0 || pos.col >= len(board[0])
}

func nextDir(dir int) int {
	return (dir + 1) % len(directions)
}

var guardIcons = []rune{'↑', '→', '↓', '←'}

func printBoard(printer *pterm.AreaPrinter, board [][]rune, pos position, dir int, visited map[position][]visitInfo) {
	if printer == nil {
		return
	}

	time.Sleep(100 * time.Millisecond)
	var b strings.Builder
	for rIdx, row := range board {
		for cIdx, r := range row {
			if rIdx == pos.row && cIdx == pos.col {
				b.WriteRune(guardIcons[dir])
			} else if vi, ok := visited[position{rIdx, cIdx}]; ok {
				b.WriteRune(guardIcons[vi[len(vi)-1].direction])
			} else {
				b.WriteRune(r)
			}
		}
		b.WriteString("\n")
	}

	printer.Update(b.String())
}
