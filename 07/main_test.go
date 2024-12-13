package main

import (
	"fmt"
	"testing"
)

var canMakeTests = []struct {
	target   int
	operands []int
	want     bool
}{
	{190, []int{10, 19}, true},
	{3267, []int{81, 40, 27}, true},
	{292, []int{11, 6, 16, 20}, true},
	{156, []int{15, 6}, false},
}

func TestCanMake(t *testing.T) {
	for _, tc := range canMakeTests {
		name := fmt.Sprintf("%v", tc)
		t.Run(name, func(t *testing.T) {
			got := CanMake(tc.target, tc.operands, false)
			if got != tc.want {
				t.Errorf("got %t, want %t", got, tc.want)
			}
		})
	}
}

var canMakeConcatTests = []struct {
	target   int
	operands []int
	want     bool
}{
	{190, []int{10, 19}, true},
	{3267, []int{81, 40, 27}, true},
	{292, []int{11, 6, 16, 20}, true},
	{156, []int{15, 6}, true},
	{7290, []int{6, 8, 6, 15}, true},
	{192, []int{17, 8, 14}, true},
	{21037, []int{9, 7, 18, 13}, false},
}

func TestCanMakeConcat(t *testing.T) {
	for _, tc := range canMakeConcatTests {
		name := fmt.Sprintf("%v", tc)
		t.Run(name, func(t *testing.T) {
			got := CanMake(tc.target, tc.operands, true)
			if got != tc.want {
				t.Errorf("got %t, want %t", got, tc.want)
			}
		})
	}
}
