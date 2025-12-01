package main

import (
	"fmt"
	"testing"
)

var grid = [][]rune{
	[]rune("....XXMAS."),
	[]rune(".SAMXMS..."),
	[]rune("...S..A..."),
	[]rune("..A.A.MS.X"),
	[]rune("XMASAMX.MM"),
	[]rune("X.....XA.A"),
	[]rune("S.S.S.S.SS"),
	[]rune(".A.A.A.A.A"),
	[]rune("..M.M.M.MM"),
	[]rune(".X.X.XMASX"),
}

var searchAroundTests = []struct{ x, y, want int }{
	{0, 0, 0},
	{4, 0, 1},
	{4, 6, 2},
	{9, 9, 2},
}

func TestSearchAround(t *testing.T) {
	for _, testcase := range searchAroundTests {
		name := fmt.Sprintf("SearchAround(%d,%d)", testcase.x, testcase.y)
		t.Run(name, func(t *testing.T) {
			got := SearchAround(grid, testcase.x, testcase.y)
			if got != testcase.want {
				t.Errorf("got %d, want %d", got, testcase.want)
			}
		})
	}
}

var part2Grid = [][]rune{
	[]rune(".M.S......"),
	[]rune("..A..MSMS."),
	[]rune(".M.S.MAA.."),
	[]rune("..A.ASMSM."),
	[]rune(".M.S.M...."),
	[]rune(".........."),
	[]rune("S.S.S.S.S."),
	[]rune(".A.A.A.A.."),
	[]rune("M.M.M.M.M."),
	[]rune(".........."),
}

var isXMasTests = []struct {
	x    int
	y    int
	want bool
}{
	{0, 0, false},
	{9, 9, false},
	{1, 2, true},
	{2, 6, true},
}

func TestIsXMas(t *testing.T) {
	for _, testcase := range isXMasTests {
		name := fmt.Sprintf("IsXMas(%d,%d)", testcase.x, testcase.y)
		t.Run(name, func(t *testing.T) {
			got := IsXMas(part2Grid, testcase.x, testcase.y)
			if got != testcase.want {
				t.Errorf("got %t, want %t", got, testcase.want)
			}
		})
	}
}
