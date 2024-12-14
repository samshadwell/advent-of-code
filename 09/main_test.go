package main

import "testing"

var input []byte = []byte("2333133121414131402")

func TestParseDiskMap(t *testing.T) {
	want := []int{
		0, 0,
		-1, -1, -1,
		1, 1, 1,
		-1, -1, -1,
		2,
		-1, -1, -1,
		3, 3, 3,
		-1,
		4, 4,
		-1,
		5, 5, 5, 5,
		-1,
		6, 6, 6, 6,
		-1,
		7, 7, 7,
		-1,
		8, 8, 8, 8,
		// No empty spaces
		9, 9,
	}
	got := ParseDiskMap(input)

	if len(want) != len(got) {
		t.Fatalf("ParseDiskMap did not return slice of expected length. want: %d, got %d", len(want), len(got))
	}
	for i, g := range got {
		if g != want[i] {
			t.Fatalf("ParseDiskMap did not return expected at position %d. want: %d got: %d", i, want[i], g)
		}
	}
}

func TestCompactDiskMap(t *testing.T) {
	got := ParseDiskMap(input)
	CompactDiskMap(got)

	want := []int{
		0, 0,
		9, 9, 8,
		1, 1, 1,
		8, 8, 8,
		2,
		7, 7, 7,
		3, 3, 3,
		6,
		4, 4,
		6,
		5, 5, 5, 5,
		6,
		6,
		-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
	}

	if len(want) != len(got) {
		t.Fatalf("CompactDiskMap did not return slice of expected length. want: %d, got %d", len(want), len(got))
	}
	for i, g := range got {
		if g != want[i] {
			t.Fatalf("CompactDiskMap did not return expected at position %d. want: %d got: %d", i, want[i], g)
		}
	}
}

func TestDefrag(t *testing.T) {
	bl := ParseToBlockList(input)
	defragged := Defrag(bl)

	want := "00992111777.44.333....5555.6666.....8888.."
	got := defragged.Render()

	if want != got {
		t.Fatalf("Defrag did not return expected result, want: %s, got %s", want, got)
	}

	wantChk := 2858
	gotChk := defragged.Checksum()
	if wantChk != gotChk {
		t.Fatalf("Checksum did not return expected result, want: %d, got %d", wantChk, gotChk)
	}
}
