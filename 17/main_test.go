package main

import (
	"strconv"
	"testing"

	"github.com/google/go-cmp/cmp"
)

var tests = []struct {
	regA, regB, regC int
	instructions     []int
	want             []int
}{
	{729, 0, 0, []int{0, 1, 5, 4, 3, 0}, []int{4, 6, 3, 5, 6, 3, 5, 2, 1, 0}},
	{117440, 0, 0, []int{0, 3, 5, 4, 3, 0}, []int{0, 3, 5, 4, 3, 0}},
}

func TestVMProcess(t *testing.T) {
	for i, tc := range tests {
		name := strconv.Itoa(i)
		t.Run(name, func(t *testing.T) {
			vm := NewVirtualMachine(tc.regA, tc.regB, tc.regC)
			got, err := vm.Process(tc.instructions)
			if err != nil {
				t.Fatalf("expected Process to return nil error, but got: %v", err)
			}

			if diff := cmp.Diff(tc.want, got); diff != "" {
				t.Fatalf("output does not match expected. Diff (-want +got): %s", diff)
			}
		})
	}
}
