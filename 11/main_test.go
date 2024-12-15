package main

import (
	"fmt"
	"testing"
)

func TestStonesAfter(t *testing.T) {
	stones := []int{125, 17}

	tests := []struct {
		numTurns int
		want     int
	}{
		{6, 22},
		{25, 55312},
	}

	for _, tc := range tests {
		name := fmt.Sprintf("%d turns", tc.numTurns)
		t.Run(name, func(t *testing.T) {
			got := StonesAfter(stones, tc.numTurns)
			if got != tc.want {
				t.Fatalf("did not receive expected number of stones. want: %d, got: %d", tc.want, got)
			}
		})
	}
}
