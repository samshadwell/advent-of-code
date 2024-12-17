package main

import (
	"strings"
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestDoMoves(t *testing.T) {
	board := [][]rune{
		[]rune("##########"),
		[]rune("#..O..O.O#"),
		[]rune("#......O.#"),
		[]rune("#.OO..O.O#"),
		[]rune("#..O...O.#"),
		[]rune("#O#..O...#"),
		[]rune("#O..O..O.#"),
		[]rune("#.OO.O.OO#"),
		[]rune("#....O...#"),
		[]rune("##########"),
	}
	moves := []rune(strings.ReplaceAll(`<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^`, "\n", ""))
	robot := Location{4, 4}

	want := `##########
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

	final, err := DoMoves(nil, board, moves, robot)
	if err != nil {
		t.Fatalf("expected DoMoves to result in no error, got: %v", err)
	}
	got := Render(board, final)
	if diff := cmp.Diff(want, got); diff != "" {
		t.Fatalf("final board does not match expected. Diff (-want +got):\n%s", diff)
	}

	wantPart1 := 10092
	gotPart1 := Part1(board)
	if gotPart1 != wantPart1 {
		t.Fatalf("score for part 1 does not match expected. Want: %d, got: %d", wantPart1, gotPart1)
	}
}
