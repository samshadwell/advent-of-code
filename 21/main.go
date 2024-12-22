package main

import (
	"advent-of-code/util/grids"
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

const (
	startButton        = 'A'
	part1Intermediates = 2
	part2Intermediates = 25
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

var dpadStart = dpad[startButton]

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening input file: %v", err)
	}

	part1 := 0
	part2 := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		code := scanner.Text()
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
	length, err := numHumanPresses(goal, numIntermediate)
	if err != nil {
		return 0, err
	}

	numeric, err := strconv.Atoi(goal[:3])
	if err != nil {
		return 0, err
	}

	return length * numeric, nil
}

// For each character of the input code, calls f with the sequence of button presses required to get there,
// summing the values returned by that callback
func sumAllRoutes(code string, keypad map[rune]grids.Location, avoid grids.Location, f func(string) (int, error)) (int, error) {
	start, ok := keypad[startButton]
	if !ok {
		return 0, fmt.Errorf("did not find start location of button %c on keypad", startButton)
	}

	curr := start
	total := 0
	for _, c := range code {
		next, ok := keypad[c]
		if !ok {
			return 0, fmt.Errorf("invalid character %c in code %s", c, code)
		}

		route := bestRoute(curr, next, avoid)
		subtotal, err := f(route)
		if err != nil {
			return 0, err
		}

		total += subtotal
		curr = next
	}
	return total, nil
}

// Total length of the sequence of button presses the human will be required to enter
// on the numIntermediate-th robot such that the given goal is typed into the 0th robot
func numHumanPresses(goal string, numIntermediate int) (int, error) {
	// Convert goal into sequence of presses on the numpad, then pass that through the n robots
	total, err := sumAllRoutes(goal, numpad, numpadGap, func(route string) (int, error) {
		return expandsInto(route, numIntermediate)
	})
	if err != nil {
		return 0, err
	}

	return total, nil
}

// Total number of button presses the given sequence expands into after being passed through n robots
func expandsInto(seq string, numIterations int) (int, error) {
	if numIterations == 0 {
		// Base case: we're not going through another robot, so the sequence won't be expanded further
		return len(seq), nil
	}

	// We'll see the same sequence at a layer many times, so cache intermediate results
	key := expandsIntoCacheKey{seq, numIterations}
	if cached, ok := expandsIntoCache[key]; ok {
		return cached, nil
	}

	// Recursive case: sum what this sequence expands into at the next layer down
	total, err := sumAllRoutes(seq, dpad, dpadGap, func(route string) (int, error) {
		return expandsInto(route, numIterations-1)
	})
	if err != nil {
		return 0, err
	}

	expandsIntoCache[key] = total
	return total, nil
}

type expandsIntoCacheKey struct {
	seq           string
	numIterations int
}

var expandsIntoCache = make(map[expandsIntoCacheKey]int)

func bestRoute(start, end, avoid grids.Location) string {
	k := bestRouteCacheKey{start, end, avoid}
	if cached, ok := bestRouteCache[k]; ok {
		return cached
	}

	delta := end.Minus(start)
	goesLeft := delta.Col < 0
	var b strings.Builder

	horizontal := func() {
		for delta.Col < 0 {
			b.WriteRune('<')
			delta.Col++
		}
		for delta.Col > 0 {
			b.WriteRune('>')
			delta.Col--
		}
	}

	vertical := func() {
		for delta.Row < 0 {
			b.WriteRune('^')
			delta.Row++
		}
		for delta.Row > 0 {
			b.WriteRune('v')
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
		// Prefer the horizontal first in 2 cases:
		// 1. Going vertical-first hits a gap
		// 2. We have to go left
		horizontal()
		vertical()
	} else {
		// Otherwise do vertical before horizontal
		vertical()
		horizontal()
	}
	b.WriteRune('A')

	res := b.String()
	bestRouteCache[k] = res
	return res
}

// There are only so many valid inputs to `bestRoute`, cache results to avoid allocating a bunch of duplicate strings
type bestRouteCacheKey struct {
	start, end, avoid grids.Location
}

var bestRouteCache = make(map[bestRouteCacheKey]string)
