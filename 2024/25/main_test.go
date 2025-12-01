package main

import (
	"strings"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestParse(t *testing.T) {
	input := `#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
`
	wantKeys := []key{
		{5, 0, 2, 1, 3},
		{4, 3, 4, 0, 2},
		{3, 0, 2, 0, 1},
	}
	wantLocks := []lock{
		{0, 5, 3, 4, 3},
		{1, 2, 0, 5, 3},
	}

	r := strings.NewReader(input)
	gotKeys, gotLocks, err := parse(r)
	if err != nil {
		t.Fatalf("wanted nil error, got: %v", err)
	}

	if diff := cmp.Diff(wantKeys, gotKeys); diff != "" {
		t.Fatalf("keys does not match expected. Diff (-want +got):\n%s", diff)
	}

	if diff := cmp.Diff(wantLocks, gotLocks); diff != "" {
		t.Fatalf("keys does not match expected. Diff (-want +got)\n%s", diff)
	}
}
