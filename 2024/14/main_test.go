package main

import (
	"fmt"
	"testing"
)

const testWidth = 11
const testHeight = 7

var positionTests = []struct{ nSeconds, wantX, wantY int }{
	{0, 2, 4},
	{1, 4, 1},
	{2, 6, 5},
}

func TestRobots(t *testing.T) {

	r, err := ParseRobot("p=2,4 v=2,-3")
	if err != nil {
		t.Fatalf("expected no error from parsing, but got: %v", err)
	}

	for _, tc := range positionTests {
		name := fmt.Sprintf("%d seconds elapsed", tc.nSeconds)
		t.Run(name, func(t *testing.T) {
			gotX, gotY := r.AfterNSeconds(tc.nSeconds, testWidth, testHeight)
			if gotX != tc.wantX || gotY != tc.wantY {
				t.Fatalf("robot not in desired position after %d seconds, want: (%d,%d), got: (%d,%d)",
					tc.nSeconds, tc.wantX, tc.wantY, gotX, gotY)
			}
		})
	}
}
