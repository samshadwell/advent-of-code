package main

import (
	"fmt"
	"testing"
)

var board = [][]rune{
	[]rune("AAAA"),
	[]rune("BBCD"),
	[]rune("BBCC"),
	[]rune("EEEC"),
}

var exploreTests = []struct {
	start Location
	want  RegionStats
}{
	{Location{0, 0}, RegionStats{area: 4, perimeter: 10, numCorners: 4}},
	{Location{1, 0}, RegionStats{area: 4, perimeter: 8, numCorners: 4}},
	{Location{1, 2}, RegionStats{area: 4, perimeter: 10, numCorners: 8}},
	{Location{1, 3}, RegionStats{area: 1, perimeter: 4, numCorners: 4}},
	{Location{3, 1}, RegionStats{area: 3, perimeter: 8, numCorners: 4}},
}

func TestExploreRegion(t *testing.T) {
	visited := make(map[Location]bool)
	for _, tc := range exploreTests {
		name := fmt.Sprintf("start at (%d,%d)", tc.start.row, tc.start.col)
		t.Run(name, func(t *testing.T) {
			got, err := ExploreRegion(board, tc.start, visited)
			if err != nil {
				t.Fatalf("wanted nil error but got %v", err)
			}

			if tc.want != got {
				t.Fatalf("want does not match got. want: %v, got: %v", tc.want, got)
			}
		})
	}
}

var board2 = [][]rune{
	[]rune("OOOOO"),
	[]rune("OXOXO"),
	[]rune("OOOOO"),
	[]rune("OXOXO"),
	[]rune("OOOOO"),
}
var part1Tests = []struct {
	board [][]rune
	want  int
}{
	{board, 140},
	{board2, 772},
	{
		board: [][]rune{
			[]rune("RRRRIICCFF"),
			[]rune("RRRRIICCCF"),
			[]rune("VVRRRCCFFF"),
			[]rune("VVRCCCJFFF"),
			[]rune("VVVVCJJCFE"),
			[]rune("VVIVCCJJEE"),
			[]rune("VVIIICJJEE"),
			[]rune("MIIIIIJJEE"),
			[]rune("MIIISIJEEE"),
			[]rune("MMMISSJEEE"),
		}, want: 1930,
	},
}

func TestPart1(t *testing.T) {
	for i, tc := range part1Tests {
		name := fmt.Sprintf("[%d]", i)
		t.Run(name, func(t *testing.T) {
			got, _ := ComputeCosts(tc.board)
			if got != tc.want {
				t.Fatalf("total cost did not match expected. want: %d, got %d", tc.want, got)
			}
		})
	}
}

var part2Tests = []struct {
	board [][]rune
	want  int
}{
	{board, 80},
	{board2, 436},
	{
		board: [][]rune{
			[]rune("EEEEE"),
			[]rune("EXXXX"),
			[]rune("EEEEE"),
			[]rune("EXXXX"),
			[]rune("EEEEE"),
		}, want: 236,
	},
	{
		board: [][]rune{
			[]rune("AAAAAA"),
			[]rune("AAABBA"),
			[]rune("AAABBA"),
			[]rune("ABBAAA"),
			[]rune("ABBAAA"),
			[]rune("AAAAAA"),
		}, want: 368,
	},
}

func TestPart2(t *testing.T) {
	for i, tc := range part2Tests {
		name := fmt.Sprintf("[%d]", i)
		t.Run(name, func(t *testing.T) {
			_, got := ComputeCosts(tc.board)
			if got != tc.want {
				t.Fatalf("total cost did not match expected. want: %d, got %d", tc.want, got)
			}
		})
	}
}
