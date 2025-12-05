package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("failed to open file: %v", err)
	}
	defer func() { _ = file.Close() }()
	scanner := bufio.NewScanner(file)

	part1 := 0
	part2 := 0
	for scanner.Scan() {
		line := scanner.Text()
		targetStr, opsStr, found := strings.Cut(line, ":")
		if !found || targetStr == "" || opsStr == "" {
			log.Fatalf("malformed input, failed to parse line into target and operands: '%s'", line)
		}

		target, err := strconv.Atoi(targetStr)
		if err != nil {
			log.Fatalf("failed to parse target string to valid number. target: '%s', err: '%v'", targetStr, err)
		}

		opsSplit := strings.Fields(opsStr)
		if len(opsSplit) == 0 {
			log.Fatal("line contains no operands")
		}

		ops := make([]int, 0, len(opsSplit))
		for _, op := range opsSplit {
			n, err := strconv.Atoi(op)
			if err != nil {
				log.Fatalf("failed to parse operand to valid number: '%s', %v", op, err)
			}
			ops = append(ops, n)
		}

		if CanMake(target, ops, false) {
			part1 += target
		}
		if CanMake(target, ops, true) {
			part2 += target
		}
	}

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}

func CanMake(target int, operands []int, withConcat bool) bool {
	if len(operands) == 0 {
		return target == 0
	}

	heads, end := operands[:len(operands)-1], operands[len(operands)-1]
	if (target-end >= 0 && CanMake(target-end, heads, withConcat)) ||
		(target%end == 0 && CanMake(target/end, heads, withConcat)) {
		return true
	}

	if prefix, ok := strings.CutSuffix(strconv.Itoa(target), strconv.Itoa(end)); withConcat && ok {
		var newTarget int
		var err error
		if prefix == "" {
			newTarget = 0
		} else {
			newTarget, err = strconv.Atoi(prefix)
			if err != nil {
				log.Fatalf("reached theroretically-impossible error converting string '%s' back to int", prefix)
			}
		}
		return CanMake(newTarget, heads, withConcat)
	}

	return false
}
