package main

import (
	"advent-of-code/util/grids"
	"bufio"
	"fmt"
	"log"
	"os"
	"runtime/pprof"
	"strconv"
)

var numpad = map[rune]grids.Location{
	'7': {Row: 0, Col: 0},
	'8': {Row: 0, Col: 1},
	'9': {Row: 0, Col: 2},
	'4': {Row: 1, Col: 0},
	'5': {Row: 1, Col: 1},
	'6': {Row: 1, Col: 2},
	'1': {Row: 2, Col: 0},
	'2': {Row: 2, Col: 1},
	'3': {Row: 2, Col: 2},
	'0': {Row: 3, Col: 1},
	'A': {Row: 3, Col: 2},
}

var dpad = map[rune]grids.Location{
	'^': {Row: 0, Col: 1},
	'A': {Row: 0, Col: 2},
	'<': {Row: 1, Col: 0},
	'v': {Row: 1, Col: 1},
	'>': {Row: 1, Col: 2},
}

var numpadGap = grids.Location{Row: 3, Col: 0}
var dpadGap = grids.Location{Row: 0, Col: 0}

const startButton = 'A'
const part1Intermediates = 2
const part2Intermediates = 20

func main() {
	// Create CPU profile file
	f, err := os.Create("cpu.prof")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	// Start CPU profiling
	if err := pprof.StartCPUProfile(f); err != nil {
		log.Fatal(err)
	}
	defer pprof.StopCPUProfile()

	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening input file: %v", err)
	}

	scanner := bufio.NewScanner(file)
	codes := make([]string, 0)
	for scanner.Scan() {
		codes = append(codes, scanner.Text())
	}

	part1 := 0
	part2 := 0
	for i, code := range codes {
		if i != 0 {
			continue
		}
		p1, err := codeComplexity(code, part1Intermediates)
		if err != nil {
			log.Fatalf("error while finding part1: %v", err)
		}
		p2, err := codeComplexity(code, part2Intermediates)
		if err != nil {
			log.Fatalf("error while finding part2: %v", err)
		}
		part1 += p1
		part2 += p2
	}

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}

func codeComplexity(goal string, numIntermediate int) (int, error) {
	route, err := findHumanPresses(goal, numIntermediate)
	if err != nil {
		return 0, err
	}

	numeric, err := strconv.Atoi(goal[:3])
	if err != nil {
		return 0, err
	}

	return len(route) * numeric, nil
}

func findHumanPresses(goal string, numIntermediate int) (string, error) {
	route := make([]rune, 0, 2<<numIntermediate)
	appendRune := func(r rune) {
		route = append(route, r)
	}

	err := routeKeypad(numpad, numpadGap, goal, appendRune)
	if err != nil {
		return "", err
	}

	for i := range numIntermediate {
		currentRoute := string(route)
		fmt.Printf("%2d: %d\n", i, len(currentRoute))
		route = route[:0] // clear but keep capacity
		err = routeKeypad(dpad, dpadGap, currentRoute, appendRune)
		if err != nil {
			return "", err
		}
	}

	return string(route), nil
}

func routeKeypad(layout map[rune]grids.Location, avoid grids.Location, goal string, emit func(rune)) error {
	loc := layout[startButton]
	for _, c := range goal {
		nextLoc, ok := layout[c]
		if !ok {
			return fmt.Errorf("invalid character in numpad goal: %c", c)
		}
		emitRoute(loc, nextLoc, avoid, emit)
		emit('A')
		loc = nextLoc
	}

	return nil
}

func emitRoute(start, end grids.Location, avoid grids.Location, emit func(rune)) {
	delta := end.Minus(start)
	goesLeft := delta.Col < 0

	horizontal := func() {
		for delta.Col < 0 {
			emit('<')
			delta.Col++
		}
		for delta.Col > 0 {
			emit('>')
			delta.Col--
		}
	}

	vertical := func() {
		for delta.Row < 0 {
			emit('^')
			delta.Row++
		}
		for delta.Row > 0 {
			emit('v')
			delta.Row--
		}
	}

	horizFirst := grids.Location{Row: start.Row, Col: end.Col}
	vertFirst := grids.Location{Row: end.Row, Col: start.Col}
	if horizFirst == avoid {
		// Going horizontal-first hits a space we should avoid so go vertical-first
		vertical()
		horizontal()
	} else if vertFirst == avoid || goesLeft {
		// Prefer the horizontal first 2 cases:
		// 1. Going vertical-first hits a gap
		// 2. We have to go left, and it's far away so we should prioritize
		horizontal()
		vertical()
	} else {
		// Otherwise do vertical before horizontal
		vertical()
		horizontal()
	}
}
