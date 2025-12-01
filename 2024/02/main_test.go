package main

import (
	"fmt"
	"testing"
)

var monotoneTests = []struct {
	in   []int
	want bool
}{
	{[]int{}, true},
	{[]int{1}, true},
	{[]int{1, 1}, false},
	{[]int{1, 2}, true},
	{[]int{1, 0}, true},
	{[]int{1, 2, 3, 100, 1000}, true},
	{[]int{10, 9, 8, 7, 6, 1}, true},
	{[]int{1, 2, 3, 2, 1}, false},
	{[]int{3, 2, 1, 2, 3}, false},
}

func TestIsStrictlyMonotone(t *testing.T) {
	for i, testcase := range monotoneTests {
		name := fmt.Sprintf("%d: %v", i, testcase.in)
		t.Run(name, func(t *testing.T) {
			got := IsStrictlyMonotone(testcase.in)
			if got != testcase.want {
				t.Errorf("got %t, want %t", got, testcase.want)
			}
		})
	}
}

var differencesTests = []struct {
	in   []int
	want bool
}{
	{[]int{}, true},
	{[]int{0}, true},
	{[]int{1, 2, 3, 2, 3, 2, 1, 4, 1}, true},
	{[]int{1, 5}, false},
	{[]int{100, 0}, false},
}

func TestHasAcceptableDifferences(t *testing.T) {
	for i, testcase := range differencesTests {
		name := fmt.Sprintf("%d: %v", i, testcase.in)
		t.Run(name, func(t *testing.T) {
			got := HasAcceptableDifferences(testcase.in)
			if got != testcase.want {
				t.Errorf("got %t, want %t", got, testcase.want)
			}
		})
	}
}

var dampeningTests = []struct {
	in   []int
	want bool
}{
	{[]int{}, true},
	{[]int{0}, true},
	{[]int{1, 3, 2, 4, 5}, true},  // Made safe by removing the 3
	{[]int{9, 7, 6, 2, 1}, false}, // No single level can be removed to make safe
}

func TestIsSafeWithDampening(t *testing.T) {
	for i, testcase := range dampeningTests {
		name := fmt.Sprintf("%d: %v", i, testcase.in)
		t.Run(name, func(t *testing.T) {
			got := IsSafeWithDampening(testcase.in)
			if got != testcase.want {
				t.Errorf("got %t, want %t", got, testcase.want)
			}
		})
	}
}
