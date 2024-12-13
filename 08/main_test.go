package main

import (
	"fmt"
	"testing"
)

var antennas = map[rune][]Location{
	'0': {{1, 8}, {2, 5}, {3, 7}, {4, 4}},
	'A': {{5, 6}, {8, 8}, {9, 9}},
}

var part1Tests = []struct {
	location Location
	want     bool
}{
	{Location{0, 0}, false},
	{Location{0, 6}, true},
}

func TestIsPart1Antinode(t *testing.T) {
	for _, tc := range part1Tests {
		name := fmt.Sprintf("%v", tc)
		t.Run(name, func(t *testing.T) {
			got := IsPart1Antinode(tc.location, antennas)
			if got != tc.want {
				t.Errorf("got %t, want %t", got, tc.want)
			}
		})
	}
}

var simplifyTests = []struct {
	in   Location
	want Location
}{
	{Location{2, 4}, Location{1, 2}},
	{Location{-1, 3}, Location{-1, 3}},
	{Location{10, 10}, Location{1, 1}},
	{Location{10, 0}, Location{1, 0}},
	{Location{0, 10}, Location{0, 1}},
}

func TestSimplifyDisplacement(t *testing.T) {
	for _, tc := range simplifyTests {
		name := fmt.Sprintf("%v", tc.in)
		t.Run(name, func(t *testing.T) {
			got := SimplifyDisplacement(tc.in)
			if got != tc.want {
				t.Errorf("got %v, want %v", got, tc.want)
			}
		})
	}

}
