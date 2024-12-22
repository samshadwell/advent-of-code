package main

import "testing"

const goal = "029A"

func Test0Intermediate(t *testing.T) {
	want := "<A^A^^>AvvvA"

	got, err := findHumanPresses(goal, 0)
	if err != nil {
		t.Fatalf("wanted nil error, but got: %v", err)
	}

	if want != got {
		t.Fatalf("did not get expected route. \nwant: %s, \ngot:  %s", want, got)
	}
}

func Test1Intermediate(t *testing.T) {
	want := "v<<A>>^A<A>A<AAv>A^A<vAAA^>A"

	got, err := findHumanPresses(goal, 1)
	if err != nil {
		t.Fatalf("wanted nil error, but got: %v", err)
	}

	if want != got {
		t.Fatalf("did not get expected route. \nwant: %s, \ngot:  %s", want, got)
	}
}

var tests = []struct {
	goal string
	want int
}{
	{"029A", 68 * 29},
	{"980A", 60 * 980},
	{"179A", 68 * 179},
	{"456A", 64 * 456},
	{"379A", 64 * 379},
}

func TestPart1Complexity(t *testing.T) {
	for _, tc := range tests {
		t.Run(tc.goal, func(t *testing.T) {
			got, err := codeComplexity(tc.goal, 2)
			if err != nil {
				t.Fatalf("wanted codeComplexity to return nil error, got: %v", err)
			}
			if tc.want != got {
				t.Fatalf("codeComplexity did not return expected result. want: %d, got:  %d", tc.want, got)
			}
		})
	}
}
