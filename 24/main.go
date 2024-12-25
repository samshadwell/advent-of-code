package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"math/rand"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

func main() {
	// f, err := os.Create("cpu.pprof")
	// if err != nil {
	// 	log.Fatal(err)
	// }
	// pprof.StartCPUProfile(f)
	// defer pprof.StopCPUProfile()

	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening file: %v", err)
	}

	system, err := ParseSystem(file)
	if err != nil {
		log.Fatalf("error while parsing system: %v", err)
	}

	part1, err := system.ToDecimal('z')
	if err != nil {
		log.Fatalf("error while evaluating input system: %v", err)
	}
	fmt.Printf("Part 1: %d\n", part1)

	badX, _ := system.ToDecimal('x')
	badY, _ := system.ToDecimal('y')
	maxX := system.MaxValue('x')
	maxY := system.MaxValue('y')

	check := func() bool {
		system.UpdateInput('x', badX)
		system.UpdateInput('y', badY)
		res, err := system.ToDecimal('z')
		if err != nil || res != badX+badX {
			return false
		}

		for range 100 {
			x := rand.Intn(maxX + 1)
			y := rand.Intn(maxY + 1)

			system.UpdateInput('x', x)
			system.UpdateInput('y', y)

			res, err := system.ToDecimal('z')
			if err != nil || res != x+y {
				return false
			}
		}
		return true
	}
	acc := make([]string, 0, 4)

	swaps := findSwaps(system, check, 1, system.AllWires(), acc)
	part2 := strings.Join(swaps, ",")

	fmt.Printf("Part 2: %s", part2)
}

type LogicGate interface {
	GetValue() (int, error)
}

type Literal struct{ val int }
type And struct {
	in1, in2 string
	sys      System
	visiting bool
}
type Or struct {
	in1, in2 string
	sys      System
	visiting bool
}
type Xor struct {
	in1, in2 string
	sys      System
	visiting bool
}

func (l *Literal) GetValue() (int, error) { return l.val, nil }
func (gate *And) GetValue() (int, error) {
	if gate.visiting {
		return 0, fmt.Errorf("cycle detected")
	}
	gate.visiting = true
	defer func() { gate.visiting = false }()

	v1, err := gate.sys[gate.in1].GetValue()
	if err != nil {
		return 0, err
	}
	v2, err := gate.sys[gate.in2].GetValue()
	if err != nil {
		return 0, err
	}

	return v1 & v2, nil
}
func (gate *Or) GetValue() (int, error) {
	if gate.visiting {
		return 0, fmt.Errorf("cycle detected")
	}
	gate.visiting = true
	defer func() { gate.visiting = false }()

	v1, err := gate.sys[gate.in1].GetValue()
	if err != nil {
		return 0, err
	}
	v2, err := gate.sys[gate.in2].GetValue()
	if err != nil {
		return 0, err
	}

	return v1 | v2, nil
}
func (gate *Xor) GetValue() (int, error) {
	if gate.visiting {
		return 0, fmt.Errorf("cycle detected")
	}
	gate.visiting = true
	defer func() { gate.visiting = false }()

	v1, err := gate.sys[gate.in1].GetValue()
	if err != nil {
		return 0, err
	}
	v2, err := gate.sys[gate.in2].GetValue()
	if err != nil {
		return 0, err
	}

	return v1 ^ v2, nil
}

type System map[string]LogicGate

func (s System) ToDecimal(prefix rune) (int, error) {
	result := 0
	for i := 0; ; i++ {
		key := fmt.Sprintf("%c%02d", prefix, i)
		gate, ok := s[key]
		if !ok {
			break
		}
		val, err := gate.GetValue()
		if err != nil {
			return 0, err
		}
		result += val << i
	}
	return result, nil
}

func (s System) AllWires() []string {
	res := make([]string, 0, len(s))
	for k := range s {
		res = append(res, k)
	}
	return res
}

func (s System) NumBits(prefix rune) int {
	for i := 0; ; i++ {
		key := fmt.Sprintf("%c%02d", prefix, i)
		if _, ok := s[key]; !ok {
			return i - 1
		}
	}
}

func (s System) MaxValue(prefix rune) int {
	numBits := s.NumBits(prefix)
	return (1<<numBits + 1) - 1
}

func (s System) UpdateInput(prefix rune, new int) {
	numBits := s.NumBits(prefix)
	max := s.MaxValue(prefix)
	if new > max || new < 0 {
		panic(fmt.Sprintf("new number out of bounds. new: %d, max: %d", new, max))
	}

	for i := 0; i <= numBits; i++ {
		bit := (new >> i) & 1
		key := fmt.Sprintf("%c%02d", prefix, i)
		gate := s[key].(*Literal)
		gate.val = bit
	}
}

var literalRegexp = regexp.MustCompile(`(\w{3}): (0|1)`)
var gateRegexp = regexp.MustCompile(`(\w{3}) (AND|OR|XOR) (\w{3}) -> (\w{3})`)

func ParseSystem(r io.Reader) (System, error) {
	sys := make(System)

	scanner := bufio.NewScanner(r)
	literalBlock := true
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			literalBlock = false
			continue
		}

		if literalBlock {
			match := literalRegexp.FindStringSubmatch(line)
			if match == nil || len(match) != 3 {
				return nil, fmt.Errorf("failed to parse line in literal block: %s, match: %v", line, match)
			}

			val, err := strconv.Atoi(match[2])
			if err != nil {
				return nil, fmt.Errorf("failed to convert literal value %s to int", match[1])
			}

			sys[match[1]] = &Literal{val: val}
		} else {
			match := gateRegexp.FindStringSubmatch(line)
			if match == nil || len(match) != 5 {
				return nil, fmt.Errorf("failed to parse line in gate block: %s, match: %v", line, match)
			}

			switch match[2] {
			case "AND":
				sys[match[4]] = &And{match[1], match[3], sys, false}
			case "OR":
				sys[match[4]] = &Or{match[1], match[3], sys, false}
			case "XOR":
				sys[match[4]] = &Xor{match[1], match[3], sys, false}
			default:
				return nil, fmt.Errorf("unrecognized gate type %s", match[2])
			}
		}
	}

	return sys, nil
}

func findSwaps(sys System, check func() bool, numSwaps int, candidates []string, soFar []string) []string {
	if numSwaps == 0 {
		if check() {
			sort.Strings(soFar)
			return soFar
		} else {
			return nil
		}
	}

	for i, s1 := range candidates {
		if s1[0] == 'x' || s1[0] == 'y' {
			continue
		}

		for _, s2 := range candidates[i+1:] {
			if s2[0] == 'x' || s2[0] == 'y' {
				continue
			}

			soFar = append(soFar, s1, s2)
			sys[s1], sys[s2] = sys[s2], sys[s1]

			res := findSwaps(sys, check, numSwaps-1, candidates[i+1:], soFar)
			if res != nil {
				return res
			}

			// Undo the swap
			sys[s1], sys[s2] = sys[s2], sys[s1]
			soFar = soFar[:len(soFar)-2]
		}
	}

	return nil
}
