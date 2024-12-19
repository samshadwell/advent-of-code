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

	"advent-of-code/util/grids"

	"github.com/pterm/pterm"
)

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
	p1Board := make([][]rune, 0)
	moves := make([]rune, 0)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			parsingBoard = false
			continue
		}

		if parsingBoard {
			p1Board = append(p1Board, []rune(line))
		} else {
			moves = append(moves, []rune(line)...)
		}
	}

	p2Board := make([][]rune, 0, len(p1Board))
	for _, row := range p1Board {
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
		p2Board = append(p2Board, row2)
	}

	_, err = DoAllMoves(area, p1Board, moves)
	if err != nil {
		log.Fatalf("error while performing part 1 moves: %v", err)
	}
	part1 := Score(p1Board)

	_, err = DoAllMoves(area, p2Board, moves)
	if err != nil {
		log.Fatalf("error while performing part 2 moves: %v", err)
	}
	part2 := Score(p2Board)

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}

func DoAllMoves(printer *pterm.AreaPrinter, board [][]rune, moves []rune) (grids.Location, error) {
	robot, ok := grids.FindRune(board, '@')
	if !ok {
		return grids.Location{}, errors.New("failed to find robot on board")
	}
	board[robot.Row][robot.Col] = '.'

	var err error
	for _, move := range moves {
		robot, err = moveRobot(board, move, robot)
		if err != nil {
			return grids.Location{}, err
		}

		if printer != nil {
			printer.Update(Render(board, robot))
			time.Sleep(time.Duration(*delayFlag) * time.Millisecond)
		}
	}
	return robot, nil
}

var left = grids.Left()
var right = grids.Right()
var up = grids.Up()
var down = grids.Down()

var directionMap = map[rune]grids.Location{
	'<': left,
	'>': right,
	'^': up,
	'v': down,
}

func moveRobot(board [][]rune, move rune, start grids.Location) (grids.Location, error) {
	direction, ok := directionMap[move]
	if !ok {
		return grids.Location{}, fmt.Errorf("unrecognized move %c", move)
	}

	next := start.Plus(direction)
	if !canVacate(board, next, direction) {
		return start, nil
	}

	if err := doVacate(board, next, direction); err != nil {
		return grids.Location{}, err
	}
	return next, nil
}

// Returns true if given location is empty, or if it can become empty by moving contents in given direction
func canVacate(board [][]rune, loc grids.Location, direction grids.Location) bool {
	contents := board[loc.Row][loc.Col]

	switch contents {
	case '.':
		return true
	case '#':
		return false
	case 'O':
		return canVacate(board, loc.Plus(direction), direction)
	case '[':
		switch direction {
		case left:
			return canVacate(board, loc.Plus(direction), direction)
		case right:
			return canVacate(board, loc.Plus(right).Plus(right), direction)
		case up, down:
			l := loc.Plus(direction)
			r := loc.Plus(direction).Plus(right)
			return canVacate(board, l, direction) && canVacate(board, r, direction)
		default:
			panic("programmer error, unknown direction in canVacate call")
		}
	case ']':
		// Right half of box simply follows left half
		return canVacate(board, loc.Plus(left), direction)
	default:
		panic(fmt.Sprintf("programmer error, default case of canVacate should be unreachable, contents %c", contents))
	}
}

// At end of function call, loc is empty and any items on it have been moved in the given direction
func doVacate(board [][]rune, loc grids.Location, direction grids.Location) error {
	contents := board[loc.Row][loc.Col]

	switch contents {
	case '.':
		return nil
	case '#':
		return errors.New("doVacate called on wall-occupied space")
	case 'O':
		next := loc.Plus(direction)
		err := doVacate(board, next, direction)
		if err != nil {
			return err
		}
		board[next.Row][next.Col] = board[loc.Row][loc.Col]
		board[loc.Row][loc.Col] = '.'
		return nil
	case ']':
		return doVacate(board, loc.Plus(left), direction)
	case '[':
		switch direction {
		case left:
			next := loc.Plus(direction)
			err := doVacate(board, next, direction)
			if err != nil {
				return err
			}
			board[next.Row][next.Col] = board[loc.Row][loc.Col]
			board[loc.Row][loc.Col] = ']'
			r := loc.Plus(right)
			board[r.Row][r.Col] = '.'
		case right:
			next := loc.Plus(direction).Plus(direction)
			err := doVacate(board, next, direction)
			if err != nil {
				return err
			}
			board[next.Row][next.Col] = ']'
			l := next.Plus(left)
			board[l.Row][l.Col] = '['
			board[loc.Row][loc.Col] = '.'
		case up, down:
			r := loc.Plus(right)
			nextL := loc.Plus(direction)
			nextR := loc.Plus(direction).Plus(right)
			err := doVacate(board, nextL, direction)
			if err != nil {
				return err
			}
			err = doVacate(board, nextR, direction)
			if err != nil {
				return err
			}
			board[nextL.Row][nextL.Col] = '['
			board[nextR.Row][nextR.Col] = ']'
			board[loc.Row][loc.Col] = '.'
			board[r.Row][r.Col] = '.'
		}
		return nil
	default:
		panic(fmt.Sprintf("programmer error, default case of doMove should be unreachable, contents %c", contents))
	}
}

func Render(board [][]rune, robot grids.Location) string {
	var sb strings.Builder
	for rIdx, row := range board {
		for cIdx, rune := range row {
			if robot.Row == rIdx && robot.Col == cIdx {
				sb.WriteRune('@')
			} else {
				sb.WriteRune(rune)
			}
		}
		sb.WriteRune('\n')
	}
	return sb.String()
}

func Score(board [][]rune) int {
	total := 0
	for rIdx, row := range board {
		for cIdx, rune := range row {
			if rune == 'O' || rune == '[' {
				total += 100*rIdx + cIdx
			}
		}
	}
	return total
}
