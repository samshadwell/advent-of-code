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

	scanner := bufio.NewScanner(file)

	numSafe := 0
	safeWithDampening := 0
	for scanner.Scan() {
		line := scanner.Text()
		split := strings.Fields(line)

		report := make([]int, len(split))
		for i, s := range split {
			parsed, err := strconv.Atoi(s)
			if err != nil {
				log.Fatalf("failed to parse string to number: %s", s)
			}
			report[i] = parsed
		}

		if IsStrictlyMonotone(report) && HasAcceptableDifferences(report) {
			numSafe += 1
			safeWithDampening += 1
		} else if IsSafeWithDampening(report) {
			safeWithDampening += 1
		}
	}
	fmt.Printf("%d reports are safe without dampening\n", numSafe)
	fmt.Printf("%d reports are safe with dampening\n", safeWithDampening)
}

func IsStrictlyMonotone(report []int) bool {
	// 0- or 1-length reports are vacuously monotone
	if len(report) < 2 {
		return true
	}

	if report[0] == report[1] {
		return false
	}

	increasing := report[1] >= report[0]
	for i := 1; i < len(report); i++ {
		if increasing && (report[i] <= report[i-1]) {
			return false
		}
		if !increasing && (report[i] >= report[i-1]) {
			return false
		}
	}
	return true
}

func HasAcceptableDifferences(report []int) bool {
	if len(report) < 2 {
		return true
	}

	for i := 1; i < len(report); i++ {
		difference := report[i] - report[i-1]
		if difference > 3 || difference < -3 {
			return false
		}
	}
	return true
}

func IsSafeWithDampening(report []int) bool {
	if len(report) == 0 {
		return true
	}
	for i := 0; i < len(report); i++ {
		newReport := withoutIndex(report, i)
		if IsStrictlyMonotone(newReport) && HasAcceptableDifferences(newReport) {
			return true
		}
	}
	return false
}

func withoutIndex(report []int, index int) []int {
	if index < 0 || index >= len(report) {
		log.Fatalf("Cannot remove index %d from slice of length %d", index, len(report))
	}
	newSlice := make([]int, 0, len(report)-1)
	newSlice = append(newSlice, report[:index]...)
	return append(newSlice, report[index+1:]...)
}
