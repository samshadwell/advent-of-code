package main

import (
	"advent-of-code/util/mymath"
	"fmt"
	"log"
	"slices"
	"strconv"
	"strings"
)

var instructions = []int{2, 4, 1, 1, 7, 5, 4, 6, 0, 3, 1, 4, 5, 5, 3, 0}

func main() {
	vm := NewVirtualMachine(28066687, 0, 0)
	output, err := vm.Process(instructions)
	if err != nil {
		log.Fatalf("error while processing program: %v", err)
	}

	outStrings := make([]string, 0, len(output))
	for _, o := range output {
		outStrings = append(outStrings, strconv.Itoa(o))
	}
	part1 := strings.Join(outStrings, ",")
	fmt.Printf("Part 1: %s\n", part1)

	// Observation 1: The length of the output is a function of A's size. We gain a digit
	// every power of 8, so for a length of 17 the input is between in the range [2^45, 2^48)
	// set n to the smallest possible value
	n := 1 << 45
	maxN := 1 << 48
	// Observation 2: The last digit output changes the most slowly. At first I thought we get a new
	// digit as position i every 2^(i + 3), but this doesn't seem to hold for sufficiently high i
	// This does seem to mean we need to adapt step size based on the number of digits which are the same, though
	maxPower := 45
	for n < maxN {
		out, _ := NewVirtualMachine(n, 0, 0).Process(instructions)
		if len(out) != len(instructions) {
			log.Fatalf("n and maxN parameters are incorrect. Expected output length (%d) to match program length (%d)", len(out), len(instructions))
		}

		numMatch := 0
		for d := len(out) - 1; d >= 0; d-- {
			if out[d] != instructions[d] {
				break
			}
			numMatch++
		}
		fmt.Printf("n: %d, numMatch: %d, out:\t%v\n", n, numMatch, out)

		if slices.Equal(out, instructions) {
			fmt.Printf("Part 2: %d\n", n)
			break
		}

		step := 1 << (maxPower - 3*numMatch)
		n += step
	}
}

type virtualMachine struct {
	pc, regA, regB, regC int
}

func NewVirtualMachine(regA, regB, regC int) *virtualMachine {
	return &virtualMachine{
		pc:   0,
		regA: regA,
		regB: regB,
		regC: regC,
	}
}

func (vm *virtualMachine) Process(program []int) ([]int, error) {
	output := make([]int, 0)

	for {
		if vm.pc+1 >= len(program) {
			break
		}
		op, arg := program[vm.pc], program[vm.pc+1]

		var err error
		switch op {
		case 0:
			err = vm.adv(arg)
		case 1:
			vm.bxl(arg)
		case 2:
			err = vm.bst(arg)
		case 3:
			didJump := vm.jnz(arg)
			if didJump {
				// Continue loop without incrementing program counter
				continue
			}
		case 4:
			vm.bxc()
		case 5:
			var o int
			o, err = vm.out(arg)
			if err == nil {
				output = append(output, o)
			}
		case 6:
			err = vm.bdv(arg)
		case 7:
			err = vm.cdv(arg)
		}
		if err != nil {
			return output, fmt.Errorf("error while executing instruction at %d", vm.pc)
		}
		vm.pc += 2
	}

	return output, nil
}

func (vm *virtualMachine) comboOp(arg int) (int, error) {
	switch arg {
	case 0, 1, 2, 3:
		return arg, nil
	case 4:
		return vm.regA, nil
	case 5:
		return vm.regB, nil
	case 6:
		return vm.regC, nil
	default:
		return 0, fmt.Errorf("invalid combo operand value %d", arg)
	}
}

func (vm *virtualMachine) adv(arg int) error {
	c, err := vm.comboOp(arg)
	if err != nil {
		return err
	}
	vm.regA = vm.regA / (1 << c)
	return nil
}
func (vm *virtualMachine) bxl(arg int) { vm.regB = vm.regB ^ arg }
func (vm *virtualMachine) bst(arg int) error {
	c, err := vm.comboOp(arg)
	if err != nil {
		return err
	}
	vm.regB = mymath.PMod(c, 8)
	return nil
}
func (vm *virtualMachine) jnz(arg int) bool {
	if vm.regA == 0 {
		return false
	}
	vm.pc = arg
	return true
}
func (vm *virtualMachine) bxc() { vm.regB = vm.regB ^ vm.regC }
func (vm *virtualMachine) out(arg int) (int, error) {
	c, err := vm.comboOp(arg)
	if err != nil {
		return 0, err
	}
	return mymath.PMod(c, 8), nil
}
func (vm *virtualMachine) bdv(arg int) error {
	c, err := vm.comboOp(arg)
	if err != nil {
		return err
	}
	vm.regB = vm.regA / (1 << c)
	return nil
}
func (vm *virtualMachine) cdv(arg int) error {
	c, err := vm.comboOp(arg)
	if err != nil {
		return err
	}
	vm.regC = vm.regA / (1 << c)
	return nil
}
