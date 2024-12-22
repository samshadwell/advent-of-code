package main

import (
	"fmt"
	"testing"
)

var tests = []struct {
	code            string
	numIntermediate int
	want            int
}{
	{"029A", 0, len("<A^A>^^AvvvA")},
	{"029A", 1, len("v<<A>>^A<A>AvA<^AA>A<vAAA>^A")},
	{"029A", 2, len("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A")},
	{"980A", 2, len("<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A")},
	{"179A", 2, len("<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A")},
	{"456A", 2, len("<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A")},
	{"379A", 2, len("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A")},
}

func TestNumHumanPresses(t *testing.T) {
	for _, tc := range tests {
		name := fmt.Sprintf("%s, %d rounds", tc.code, tc.numIntermediate)
		t.Run(name, func(t *testing.T) {
			got, err := numHumanPresses(tc.code, tc.numIntermediate)
			if err != nil {
				t.Fatalf("wanted codeComplexity to return nil error, got: %v", err)
			}
			if tc.want != got {
				t.Fatalf("codeComplexity did not return expected result. want: %d, got:  %d", tc.want, got)
			}
		})
	}
}
