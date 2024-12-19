package mymath

import "fmt"

// (P)ositive (Mod)ulus. The modulo operator as pretty much every other programming
// language knows it. Output is non-negative, panics if d <= 0
func PMod(n, d int) int {
	if d <= 0 {
		panic(fmt.Sprintf("PMod expects a strictly positive modulus, given %d", d))
	}
	mod := n % d
	if mod < 0 {
		return mod + d
	}
	return mod
}
