package main

import (
	"fmt"
	"testing"
)

var tests = []struct {
	maze           [][]rune
	wantP1, wantP2 int
}{
	{
		maze: [][]rune{
			[]rune("###############"),
			[]rune("#.......#....E#"),
			[]rune("#.#.###.#.###.#"),
			[]rune("#.....#.#...#.#"),
			[]rune("#.###.#####.#.#"),
			[]rune("#.#.#.......#.#"),
			[]rune("#.#.#####.###.#"),
			[]rune("#...........#.#"),
			[]rune("###.#.#####.#.#"),
			[]rune("#...#.....#.#.#"),
			[]rune("#.#.#.###.#.#.#"),
			[]rune("#.....#...#.#.#"),
			[]rune("#.###.#.#.#.#.#"),
			[]rune("#S..#.....#...#"),
			[]rune("###############"),
		},
		wantP1: 7036,
		wantP2: 45,
	},
	{
		maze: [][]rune{
			[]rune("#################"),
			[]rune("#...#...#...#..E#"),
			[]rune("#.#.#.#.#.#.#.#.#"),
			[]rune("#.#.#.#...#...#.#"),
			[]rune("#.#.#.#.###.#.#.#"),
			[]rune("#...#.#.#.....#.#"),
			[]rune("#.#.#.#.#.#####.#"),
			[]rune("#.#...#.#.#.....#"),
			[]rune("#.#.#####.#.###.#"),
			[]rune("#.#.#.......#...#"),
			[]rune("#.#.###.#####.###"),
			[]rune("#.#.#...#.....#.#"),
			[]rune("#.#.#.#####.###.#"),
			[]rune("#.#.#.........#.#"),
			[]rune("#.#.#.#########.#"),
			[]rune("#S#.............#"),
			[]rune("#################"),
		},
		wantP1: 11048,
		wantP2: 64,
	},
}

func TestFindBestPath(t *testing.T) {
	for i, tc := range tests {
		name := fmt.Sprintf("%d", i)
		t.Run(name, func(t *testing.T) {
			gotP1, gotP2, err := findBestPath(tc.maze)
			if err != nil {
				t.Fatalf("wanted nil error, but got: %v", err)
			}
			if tc.wantP1 != gotP1 {
				t.Fatalf("wanted score of %d, but got: %d", tc.wantP1, gotP1)
			}
			if tc.wantP2 != gotP2 {
				t.Fatalf("wanted tile count of %d, but got: %d", tc.wantP2, gotP2)
			}
		})
	}

}
