package main

import (
	"strings"
	"testing"

	"github.com/google/go-cmp/cmp"
)

var part1Board = [][]rune{
	[]rune("##########"),
	[]rune("#..O..O.O#"),
	[]rune("#......O.#"),
	[]rune("#.OO..O.O#"),
	[]rune("#..O@..O.#"),
	[]rune("#O#..O...#"),
	[]rune("#O..O..O.#"),
	[]rune("#.OO.O.OO#"),
	[]rune("#....O...#"),
	[]rune("##########"),
}

var part2Board = [][]rune{
	[]rune("####################"),
	[]rune("##....[]....[]..[]##"),
	[]rune("##............[]..##"),
	[]rune("##..[][]....[]..[]##"),
	[]rune("##....[]@.....[]..##"),
	[]rune("##[]##....[]......##"),
	[]rune("##[]....[]....[]..##"),
	[]rune("##..[][]..[]..[][]##"),
	[]rune("##........[]......##"),
	[]rune("####################"),
}

var part1Rendered = `##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
`

var part2Rendered = `####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################
`

var testMoves = []rune(strings.ReplaceAll(`<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^`, "\n", ""))

var tests = []struct {
	name         string
	board        [][]rune
	wantRendered string
	wantScore    int
}{
	{"Part 1", part1Board, part1Rendered, 10092},
	{"Part 2", part2Board, part2Rendered, 9021},
}

func TestDoAllMoves(t *testing.T) {
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			final, err := DoAllMoves(nil, tc.board, testMoves)
			if err != nil {
				t.Fatalf("expected DoMoves to result in no error, got: %v", err)
			}
			got := Render(tc.board, final)
			if diff := cmp.Diff(tc.wantRendered, got); diff != "" {
				t.Fatalf("final rendered board does not match expected. Diff (-want +got):\n%s", diff)
			}

			gotScore := Score(tc.board)
			if gotScore != tc.wantScore {
				t.Fatalf("score does not match expected. Want: %d, got: %d", tc.wantScore, gotScore)
			}
		})
	}
}
