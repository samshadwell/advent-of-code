package main

import (
	"fmt"
	"testing"
)

var solveTests = []struct {
	m            Machine
	wantA, wantB int
	shouldSolve  bool
}{
	{
		m: Machine{
			94, 34,
			22, 67,
			8400, 5400,
		},
		wantA: 80, wantB: 40, shouldSolve: true,
	},
	{
		m: Machine{
			26, 66,
			67, 21,
			12748, 12176,
		},
		wantA: 0, wantB: 0, shouldSolve: false,
	},
	{
		m: Machine{
			17, 86,
			84, 37,
			7870, 6450,
		},
		wantA: 38, wantB: 86, shouldSolve: true,
	},
	{
		m: Machine{
			69, 23,
			27, 71,
			18641, 10279,
		},
		wantA: 0, wantB: 0, shouldSolve: false,
	},
}

func TestMachineSolve(t *testing.T) {
	for _, tc := range solveTests {
		name := fmt.Sprintf("%v", tc.m)
		t.Run(name, func(t *testing.T) {
			gotA, gotB, solves := tc.m.Solve()
			if gotA != tc.wantA || gotB != tc.wantB || solves != tc.shouldSolve {
				t.Fatalf("solution did not match expected. want: (%d, %d, %t), got: (%d, %d, %t)",
					tc.wantA, tc.wantB, tc.shouldSolve,
					gotA, gotB, solves)
			}
		})
	}
}
