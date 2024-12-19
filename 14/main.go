package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"

	"advent-of-code/util/grids"
	"advent-of-code/util/mymath"
)

type Robot struct {
	X, Y, Vx, Vy int
}

const width = 101
const height = 103

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error opening input file: %v", err)
	}

	robots := make([]Robot, 0)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		r, err := ParseRobot(line)
		if err != nil {
			log.Fatalf("error while parsing input: %v", err)
		}
		robots = append(robots, r)
	}

	quadCounts := make([]int, 4)
	for _, r := range robots {
		x, y := r.AfterNSeconds(100, width, height)
		if x < width/2 && y < height/2 {
			quadCounts[0]++
		} else if x > width/2 && y < height/2 {
			quadCounts[1]++
		} else if x < width/2 && y > height/2 {
			quadCounts[2]++
		} else if x > width/2 && y > height/2 {
			quadCounts[3]++
		}
	}

	n := 0
	for ; true; n++ {
		uniquePositions := make(map[grids.Location]bool)
		for _, r := range robots {
			x, y := r.AfterNSeconds(n, width, height)
			uniquePositions[grids.Location{Row: y, Col: x}] = true
		}

		// A bit cheeky... I guessed that all the robots would be in unique positions iff they were in the easter egg
		// formation. Turns out to be correct for my input!
		if len(uniquePositions) == len(robots) {
			s := Plot(robots, n, width, height)
			fmt.Println(s)
			break
		}
	}

	part1 := quadCounts[0] * quadCounts[1] * quadCounts[2] * quadCounts[3]

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", n)
}

var re = regexp.MustCompile(`p=(\d+),(\d+) v=(-?\d+),(-?\d+)`)

func ParseRobot(in string) (Robot, error) {
	match := re.FindStringSubmatch(in)
	if match == nil {
		return Robot{}, fmt.Errorf("could not parse string '%s' into valid robot", in)
	}

	if len(match) != 5 {
		return Robot{}, fmt.Errorf("programmer error, expected match to contain exactly 5 elements, got %v", match)
	}

	parts := make([]int, 0, 4)
	for i := 1; i < len(match); i++ {
		n, err := strconv.Atoi(match[i])
		if err != nil {
			return Robot{}, fmt.Errorf("error parsing string '%s' to int: %w", match[i], err)
		}
		parts = append(parts, n)
	}

	return Robot{
		parts[0], parts[1], parts[2], parts[3],
	}, nil
}

func (r *Robot) AfterNSeconds(n, width, height int) (int, int) {
	return mymath.PMod(n*r.Vx+r.X, width), mymath.PMod(n*r.Vy+r.Y, height)
}

func Plot(robots []Robot, secondsElapsed, width, height int) string {
	positions := make(map[grids.Location]int)
	for _, r := range robots {
		x, y := r.AfterNSeconds(secondsElapsed, width, height)
		positions[grids.Location{Row: y, Col: x}]++
	}

	var sb strings.Builder
	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			count := positions[grids.Location{Row: y, Col: x}]
			if count == 0 {
				sb.WriteRune('.')
			} else if count < 10 {
				sb.WriteString(strconv.Itoa(count))
			} else {
				sb.WriteRune('+')
			}
		}
		sb.WriteString("\n")
	}
	return sb.String()
}
