package main

import (
	"bufio"
	"errors"
	"flag"
	"fmt"
	"log"
	"os"
	"strings"
	"time"

	"github.com/pterm/pterm"
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

var inputFlag = flag.String("file", "input.txt", "File to read as input")
var printFlag = flag.Bool("print", false, "Set to display simulation of walking")
var delayFlag = flag.Int("delay", 50, "When printing, delay between frames in milliseconds")

func main() {
	flag.Parse()
	file, err := os.Open(*inputFlag)
	if err != nil {
		log.Fatalf("error while opening input file %s: %v", *inputFlag, err)
	}
	scanner := bufio.NewScanner(file)

	var area *pterm.AreaPrinter
	if *printFlag {
		area, err = pterm.DefaultArea.WithCenter().Start()
		if err != nil {
			log.Fatal("Failed to start output area")
		}
	}

	parsingBoard := true
	board := make([][]rune, 0)
	moves := make([]rune, 0)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			parsingBoard = false
			continue
		}

		if parsingBoard {
			board = append(board, []rune(line))
		} else {
			moves = append(moves, []rune(line)...)
		}
	}

	board2 := make([][]rune, 0, len(board))
	for _, row := range board {
		row2 := make([]rune, 0, 2*len(row))
		for _, rune := range row {
			if rune == '#' {
				row2 = append(row2, '#', '#')
			} else if rune == 'O' {
				row2 = append(row2, '[', ']')
			} else if rune == '.' {
				row2 = append(row2, '.', '.')
			} else if rune == '@' {
				row2 = append(row2, '@', '.')
			}
		}
		board2 = append(board2, row2)
	}

	robot, err := FindAndReplaceRobot(board)
	if err != nil {
		log.Fatalf("error while finding robot: %v", err)
	}
	_, err = DoMoves(area, board, moves, robot)
	if err != nil {
		log.Fatalf("error while performing moves: %v", err)
	}

	part1 := Part1(board)

	fmt.Printf("Part 1: %d\n", part1)

	robot2, _ := FindAndReplaceRobot(board2)
	fmt.Println(Render(board2, robot2))
}

func FindAndReplaceRobot(board [][]rune) (Location, error) {
	for rIdx, row := range board {
		for cIdx, rune := range row {
			if rune == '@' {
				board[rIdx][cIdx] = '.'
				return Location{rIdx, cIdx}, nil
			}
		}
	}
	return Location{}, errors.New("failed to find robot on board")
}

func DoMoves(printer *pterm.AreaPrinter, board [][]rune, moves []rune, robot Location) (Location, error) {
	var err error
	for _, move := range moves {
		robot, err = MoveRobot(board, move, robot)
		if err != nil {
			return Location{}, err
		}

		if printer != nil {
			time.Sleep(time.Millisecond * time.Duration(*delayFlag))
			printer.Update(Render(board, robot))
		}
	}
	return robot, nil
}

var moves = map[rune]Location{
	'v': {row: 1, col: 0},
	'>': {row: 0, col: 1},
	'^': {row: -1, col: 0},
	'<': {row: 0, col: -1},
}

func MoveRobot(board [][]rune, move rune, start Location) (Location, error) {
	direction, ok := moves[move]
	if !ok {
		return Location{}, fmt.Errorf("unrecognized move direction '%c'", move)
	}

	nextLocation := start.Plus(direction)
	nextContents := board[nextLocation.row][nextLocation.col]
	if nextContents == '.' {
		// Case 1: space is empty, robot simply moves into it
		return nextLocation, nil
	} else if nextContents == '#' {
		// Case 2: space is a wall, robot fails to move
		return start, nil
	}

	// Error case, assuming that a "valid" board should only have one of three runes
	if nextContents != 'O' {
		return Location{}, fmt.Errorf("unrecognized board contents: '%c'", nextContents)
	}

	// Case 3: space is a box. We may either push it or be blocked
	eventual := nextLocation.Plus(direction)
	for ; board[eventual.row][eventual.col] == 'O'; eventual = eventual.Plus(direction) {
	}
	eventualContents := board[eventual.row][eventual.col]

	if eventualContents == '.' {
		// Case 3a: we push box(es) into an empty space
		board[eventual.row][eventual.col] = 'O'
		board[nextLocation.row][nextLocation.col] = '.'
		return nextLocation, nil
	} else if eventualContents == '#' {
		// Case 3b: we're pushing on a box(es) against a wall, we fail to move
		return start, nil
	} else {
		return Location{}, fmt.Errorf("programmer error, unexpected rune when looking down boxes: '%c'", eventualContents)
	}
}

func Render(board [][]rune, robot Location) string {
	var sb strings.Builder
	for rIdx, row := range board {
		for cIdx, rune := range row {
			if robot.row == rIdx && robot.col == cIdx {
				sb.WriteRune('@')
			} else {
				sb.WriteRune(rune)
			}
		}
		sb.WriteString("\n")
	}
	return sb.String()
}

func Part1(board [][]rune) int {
	total := 0
	for rIdx, row := range board {
		for cIdx, rune := range row {
			if rune == 'O' {
				total += 100*rIdx + cIdx
			}
		}
	}
	return total
}
