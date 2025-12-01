package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	input, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatalf("error while reading file: %v", err)
	}

	split := strings.Fields(string(input))
	stones := make([]int, 0, len(split))
	for _, s := range split {
		n, err := strconv.Atoi(s)
		if err != nil {
			log.Fatalf("error converting string to number '%s': %v", s, err)
		}
		stones = append(stones, n)
	}

	fmt.Printf("Part 1: %d\n", StonesAfter(stones, 25))
	fmt.Printf("Part 2: %d\n", StonesAfter(stones, 75))
}

func StonesAfter(stones []int, numTurns int) int {
	total := 0
	memo := make(map[args]int)
	for _, stone := range stones {
		total += stonesAfterHelper(stone, numTurns, memo)
	}
	return total
}

type args struct {
	number   int
	numTurns int
}

func stonesAfterHelper(number int, numTurns int, memo map[args]int) int {
	if numTurns == 0 {
		return 1
	}

	a := args{number, numTurns}
	if res, ok := memo[a]; ok {
		return res
	}

	var res int
	if number == 0 {
		res = stonesAfterHelper(1, numTurns-1, memo)
	} else if asString := strconv.Itoa(number); len(asString)%2 == 0 {
		halfway := len(asString) / 2
		l, r := asString[:halfway], asString[halfway:]

		left, err := strconv.Atoi(l)
		if err != nil {
			log.Fatalf("error converting '%s' back to integer, should be impossible: %v", l, err)
		}

		right, err := strconv.Atoi(r)
		if err != nil {
			log.Fatalf("error converting '%s' back to integer, should be impossible: %v", r, err)
		}

		res = stonesAfterHelper(left, numTurns-1, memo) + stonesAfterHelper(right, numTurns-1, memo)
	} else {
		res = stonesAfterHelper(number*2024, numTurns-1, memo)
	}

	memo[a] = res
	return res
}
