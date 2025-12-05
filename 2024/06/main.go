package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"runtime/pprof"
	"strings"
	"time"

	"advent-of-code/util/grids"

	"github.com/pterm/pterm"
)

var directions = []grids.Location{
	grids.Up(),
	grids.Right(),
	grids.Down(),
	grids.Left(),
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
		board[pos.Row][pos.Col] = '#'
		loopRun := run(nil, board, start)
		if !loopRun.terminates {
			part2 += 1
		}
		board[pos.Row][pos.Col] = '.'
	}

	fmt.Printf("Part 1: %d\n", len(baseRun.squaresVisited))
	fmt.Printf("Part 2: %d\n", part2)

	if *memprofile != "" {
		f, err := os.Create(*memprofile)
		if err != nil {
			log.Fatal("could not create memory profile: ", err)
		}
		defer func() { _ = f.Close() }()
		if err := pprof.WriteHeapProfile(f); err != nil {
			log.Fatal("could not write memory profile: ", err)
		}
	}
}

func parseInput() ([][]rune, grids.Location) {
	file, err := os.Open(*inputFlag)
	if err != nil {
		log.Fatalf("failed to open file: %v", err)
	}
	defer func() { _ = file.Close() }()
	board := grids.ParseRuneGrid(file)
	start, ok := grids.FindRune(board, '^')
	if !ok {
		log.Fatalf("failed to find guard position")
	}
	board[start.Row][start.Col] = '.'

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
	squaresVisited map[grids.Location][]visitInfo
	terminates     bool
}

type visitInfo struct {
	direction int
	sequence  int
}

func run(printer *pterm.AreaPrinter, board [][]rune, start grids.Location) runResult {
	pos := start
	dir := 0
	sequence := 0
	visited := make(map[grids.Location][]visitInfo)

	for {
		sequence += 1
		printBoard(printer, board, pos, dir, visited)

		if _, ok := visited[pos]; !ok {
			visited[pos] = make([]visitInfo, 0)
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

		newPos := pos.Plus(directions[dir])

		if grids.IsOutOfBounds(newPos, len(board), len(board[0])) {
			break
		}

		if board[newPos.Row][newPos.Col] == '#' {
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

func nextDir(dir int) int {
	return (dir + 1) % len(directions)
}

var guardIcons = []rune{'↑', '→', '↓', '←'}

func printBoard(printer *pterm.AreaPrinter, board [][]rune, pos grids.Location, dir int, visited map[grids.Location][]visitInfo) {
	if printer == nil {
		return
	}

	time.Sleep(100 * time.Millisecond)
	var b strings.Builder
	for rIdx, row := range board {
		for cIdx, r := range row {
			if rIdx == pos.Row && cIdx == pos.Col {
				b.WriteRune(guardIcons[dir])
			} else if vi, ok := visited[grids.Location{Row: rIdx, Col: cIdx}]; ok {
				b.WriteRune(guardIcons[vi[len(vi)-1].direction])
			} else {
				b.WriteRune(r)
			}
		}
		b.WriteRune('\n')
	}
	printer.Update(b.String())
}
