package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

type Machine struct {
	Ax, Ay, Bx, By, Px, Py int
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error opening input: %v", err)
	}
	scanner := bufio.NewScanner(file)
	machines := make([]Machine, 0)
	for m, ok := parseInput(scanner); ok; m, ok = parseInput(scanner) {
		machines = append(machines, m)
	}

	part1 := 0
	part2 := 0
	for _, m := range machines {
		a, b, solves := m.Solve()
		if solves {
			part1 += 3*a + b
		}

		m.Px += 10000000000000
		m.Py += 10000000000000
		a, b, solves = m.Solve()
		if solves {
			part2 += 3*a + b
		}
	}

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}

var aRegex = regexp.MustCompile(`Button A: X\+(\d+), Y\+(\d+)`)
var bRegex = regexp.MustCompile(`Button B: X\+(\d+), Y\+(\d+)`)
var pRegex = regexp.MustCompile(`Prize: X=(\d+), Y=(\d+)`)

func parseInput(scanner *bufio.Scanner) (Machine, bool) {
	lines := make([]string, 0, 3)
	for i := 0; i < 3; i++ {
		ok := scanner.Scan()
		if !ok {
			return Machine{}, false
		}
		lines = append(lines, scanner.Text())
	}
	_ = scanner.Scan()

	ax, ay, err := extractValues(lines[0], aRegex)
	if err != nil {
		return Machine{}, false
	}
	bx, by, err := extractValues(lines[1], bRegex)
	if err != nil {
		return Machine{}, false
	}
	px, py, err := extractValues(lines[2], pRegex)
	if err != nil {
		return Machine{}, false
	}

	return Machine{
		Ax: ax, Ay: ay,
		Bx: bx, By: by,
		Px: px, Py: py,
	}, true
}

func extractValues(line string, re *regexp.Regexp) (int, int, error) {
	match := re.FindStringSubmatch(line)
	if match == nil {
		return 0, 0, fmt.Errorf("regexp '%s' did not match line '%s'", re.String(), line)
	}
	if len(match) != 3 {
		return 0, 0, fmt.Errorf("programmer error, extractValues expects regexp to have exactly 2 capture groups, got: %s", re.String())
	}

	x, err := strconv.Atoi(match[1])
	if err != nil {
		return 0, 0, err
	}

	y, err := strconv.Atoi(match[2])
	if err != nil {
		return 0, 0, err
	}

	return x, y, nil
}

func (m *Machine) Solve() (int, int, bool) {
	// Via matrix algebra, inverting the 2x2 matrix. See: https://www.chilimath.com/lessons/advanced-algebra/inverse-of-a-2x2-matrix/
	det := m.Ax*m.By - m.Ay*m.Bx
	if det == 0 {
		return 0, 0, false
	}

	aNum := m.By*m.Px - m.Bx*m.Py
	bNum := -m.Ay*m.Px + m.Ax*m.Py

	// Note: May truncate in case of non-integer solution
	a := aNum / det
	b := bNum / det

	// Ensure solution is positive integer number of presses
	if aNum != a*det || bNum != b*det || a < 0 || b < 0 {
		return 0, 0, false
	}

	return a, b, true
}
