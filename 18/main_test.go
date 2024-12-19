package main

import (
	"advent-of-code/util/grids"
	"testing"
)

func TestBfs(t *testing.T) {
	corrupt := []grids.Location{
		{Col: 5, Row: 4},
		{Col: 4, Row: 2},
		{Col: 4, Row: 5},
		{Col: 3, Row: 0},
		{Col: 2, Row: 1},
		{Col: 6, Row: 3},
		{Col: 2, Row: 4},
		{Col: 1, Row: 5},
		{Col: 0, Row: 6},
		{Col: 3, Row: 3},
		{Col: 2, Row: 6},
		{Col: 5, Row: 1},
	}
	start := grids.Location{Col: 0, Row: 0}
	goal := grids.Location{Col: 6, Row: 6}
	want := 22

	path, err := bfs(corrupt, start, goal)
	got := len(path) - 1
	if err != nil {
		t.Fatalf("wanted part1 to return nil error, got: %v", err)
	}

	if want != got {
		t.Fatalf("call to part1 did not return expected value. want: %d, got: %d", want, got)
	}
}
