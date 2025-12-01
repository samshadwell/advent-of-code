package main

import (
	"fmt"
	"testing"
)

var prngTests = []struct{ seed, n, want int }{
	{123, 1, 15887950},
	{123, 2, 16495136},
	{123, 3, 527345},
	{123, 4, 704524},
	{123, 5, 1553684},
	{123, 6, 12683156},
	{123, 7, 11100544},
	{123, 8, 12249484},
	{123, 9, 7753432},
	{123, 10, 5908254},
	{1, 2000, 8685429},
	{10, 2000, 4700978},
	{100, 2000, 15273692},
	{2024, 2000, 8667524},
}

func TestPrng(t *testing.T) {
	for _, tc := range prngTests {
		name := fmt.Sprintf("seed=%d, n=%d", tc.seed, tc.n)
		t.Run(name, func(t *testing.T) {
			prng := NewMonkeyPrng(tc.seed)
			got := prng.Nth(tc.n)
			if got != tc.want {
				t.Fatalf("did not get expected random number. got: %d, want: %d", got, tc.want)
			}
		})
	}
}

var allSalePriceTests = []struct {
	secret        int
	numIterations int
	testKey       quad
	want          int
}{
	{123, 9, quad{-1, -1, 0, 2}, 6},
	{1, 2000, quad{-2, 1, -1, 3}, 7},
	{2, 2000, quad{-2, 1, -1, 3}, 7},
	{3, 2000, quad{-2, 1, -1, 3}, 0},
	{2024, 2000, quad{-2, 1, -1, 3}, 9},
}

func TestAllSalePrices(t *testing.T) {
	for _, tc := range allSalePriceTests {
		name := fmt.Sprintf("%v", tc)
		t.Run(name, func(t *testing.T) {
			prices := allSalePrices(tc.secret, tc.numIterations)
			got := prices[tc.testKey]
			if got != tc.want {
				t.Fatalf("returned value did not match expected. want: %d, got: %d", tc.want, got)
			}
		})
	}
}
