package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
)

const (
	pinsPerLock   = 5
	lockHeight    = 5
	linesPerInput = lockHeight + 2
)

type key []int
type lock []int

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening input file: %v", err)
	}

	keys, locks, err := parse(file)
	if err != nil {
		log.Fatalf("error while parsing input: %v", err)
	}

	combinations := 0
	for _, k := range keys {
		for _, l := range locks {
			if compatible(k, l) {
				combinations++
			}
		}
	}

	fmt.Printf("Part 1: %d\n", combinations)
}

func parse(r io.Reader) ([]key, []lock, error) {
	keys := make([]key, 0)
	locks := make([]lock, 0)

	scanner := bufio.NewScanner(r)
	for {
		key, lock, err := parseOne(scanner)
		if err != nil {
			return nil, nil, err
		}

		if key != nil {
			keys = append(keys, key)
		} else if lock != nil {
			locks = append(locks, lock)
		} else {
			break
		}
	}

	return keys, locks, nil
}

func parseOne(scanner *bufio.Scanner) (key, lock, error) {
	lines := make([]string, 0, linesPerInput)
	for range linesPerInput {
		ok := scanner.Scan()
		if !ok {
			return nil, nil, nil
		}
		line := scanner.Text()
		if len(line) != pinsPerLock {
			return nil, nil, fmt.Errorf("unexpected number of characters in line %s, expected %d", line, pinsPerLock)
		}
		lines = append(lines, scanner.Text())
	}
	_ = scanner.Scan()

	res := make([]int, 0, pinsPerLock)
	isKey := lines[0][0] != '#'
	for pin := 0; pin < pinsPerLock; pin++ {
		for height := 0; height <= lockHeight; height++ {
			var rIdx int
			if isKey {
				rIdx = lockHeight - height
			} else {
				rIdx = 1 + height
			}

			if lines[rIdx][pin] == '.' {
				res = append(res, height)
				break
			}
		}
	}

	if len(res) != pinsPerLock {
		return nil, nil, fmt.Errorf("programmer error, not enough pin heights in resulting key/lock")
	}

	if isKey {
		return res, nil, nil
	} else {
		return nil, res, nil
	}
}

func compatible(k key, l lock) bool {
	for i := range pinsPerLock {
		if k[i]+l[i] > lockHeight {
			return false
		}
	}
	return true
}
