package main

import (
	"fmt"
	"testing"

	"advent-of-code/util/grids"
)

var antennas = map[rune][]grids.Location{
	'0': {{Row: 1, Col: 8}, {Row: 2, Col: 5}, {Row: 3, Col: 7}, {Row: 4, Col: 4}},
	'A': {{Row: 5, Col: 6}, {Row: 8, Col: 8}, {Row: 9, Col: 9}},
}

var part1Tests = []struct {
	location grids.Location
	want     bool
}{
	{grids.Location{Row: 0, Col: 0}, false},
	{grids.Location{Row: 0, Col: 6}, true},
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
	in   grids.Location
	want grids.Location
}{
	{grids.Location{Row: 2, Col: 4}, grids.Location{Row: 1, Col: 2}},
	{grids.Location{Row: -1, Col: 3}, grids.Location{Row: -1, Col: 3}},
	{grids.Location{Row: 10, Col: 10}, grids.Location{Row: 1, Col: 1}},
	{grids.Location{Row: 10, Col: 0}, grids.Location{Row: 1, Col: 0}},
	{grids.Location{Row: 0, Col: 10}, grids.Location{Row: 0, Col: 1}},
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
