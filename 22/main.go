package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening input file: %v", err)
	}

	scanner := bufio.NewScanner(file)
	secrets := make([]int, 0)
	for scanner.Scan() {
		line := scanner.Text()
		secret, err := strconv.Atoi(line)
		if err != nil {
			log.Fatalf("error converting input line %s to number: %v", line, err)
		}
		secrets = append(secrets, secret)
	}

	part1 := 0
	for _, secret := range secrets {
		prng := NewMonkeyPrng(secret)
		part1 += prng.Nth(2000)
	}
	fmt.Printf("Part 1: %d\n", part1)

	cumulativePrices := make(map[quad]int)
	for _, s := range secrets {
		salePrices := allSalePrices(s, 2000)
		for q, p := range salePrices {
			cumulativePrices[q] += p
		}
	}

	bestQ := quad{}
	part2 := 0
	for q, p := range cumulativePrices {
		if p > part2 {
			bestQ = q
			part2 = p
		}
	}

	fmt.Printf("Part 2: %v yields %d bananas\n", bestQ, part2)
}

type quad struct {
	v0, v1, v2, v3 int
}

func allSalePrices(secret, numIterations int) map[quad]int {
	salePrices := make(map[quad]int)
	prng := NewMonkeyPrng(secret)

	allNumbers := make([]int, 0, numIterations+1)
	allNumbers = append(allNumbers, prng.secret%10)
	for range numIterations {
		allNumbers = append(allNumbers, prng.Next()%10)
	}

	for _, window := range slidingWindow(5, allNumbers) {
		changes := quad{
			window[1] - window[0],
			window[2] - window[1],
			window[3] - window[2],
			window[4] - window[3],
		}
		if _, ok := salePrices[changes]; ok {
			// We've already sold for the quad representing this set of changes
			continue
		}

		salePrices[changes] = window[4]
	}

	return salePrices
}

func slidingWindow(size int, input []int) [][]int {
	// returns the input slice as the first element
	if len(input) <= size {
		return [][]int{input}
	}

	// allocate slice at the precise size we need
	r := make([][]int, 0, len(input)-size+1)

	for i, j := 0, size; j <= len(input); i, j = i+1, j+1 {
		r = append(r, input[i:j])
	}

	return r
}

const (
	pruneMask = 16777216 - 1
)

type monkeyPrng struct {
	secret int
}

func NewMonkeyPrng(seed int) *monkeyPrng {
	return &monkeyPrng{seed}
}

func (m *monkeyPrng) Nth(n int) int {
	for range n - 1 {
		_ = m.Next()
	}
	return m.Next()
}

func (m *monkeyPrng) Next() int {
	m.mix(m.secret << 6)
	m.prune()
	m.mix(m.secret >> 5)
	m.prune()
	m.mix(m.secret << 11)
	m.prune()
	return m.secret
}

func (m *monkeyPrng) mix(n int) { m.secret ^= n }
func (m *monkeyPrng) prune()    { m.secret &= pruneMask }
