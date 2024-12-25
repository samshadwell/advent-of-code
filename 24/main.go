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
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening file: %v", err)
	}

	system, err := ParseSystem(file)
	if err != nil {
		log.Fatalf("error while parsing system: %v", err)
	}

	part1 := system.Output()
	if err != nil {
		log.Fatalf("error while evaluating input system: %v", err)
	}
	fmt.Printf("Part 1: %d\n", part1)

	// fmt.Println(system.DotGraph())

	// This part was derived by hand. My methodology was to use the below `check` function
	// to find where in the graph the first error is (where the error bits are no longer 0),
	// then look at the dot-graph (commented-out above) to look and find the aberration
	swaps := [][]string{
		{"fkb", "z16"},
		{"nnr", "rqf"},
		{"z31", "rdn"},
		{"z37", "rrn"},
	}
	for _, s := range swaps {
		system[s[0]], system[s[1]] = system[s[1]], system[s[0]]
	}

	if !check(system) {
		return
	}

	flatSwaps := make([]string, 0, 8)
	for _, s := range swaps {
		flatSwaps = append(flatSwaps, s...)
	}
	sort.Strings(flatSwaps)
	part2 := strings.Join(flatSwaps, ",")
	fmt.Printf("Part 2: %s\n", part2)
}

type LogicGate interface {
	GetValue() int
}

type Literal struct{ val int }
type And struct {
	in1, in2 string
	sys      System
}
type Or struct {
	in1, in2 string
	sys      System
}
type Xor struct {
	in1, in2 string
	sys      System
}

func (l *Literal) GetValue() int { return l.val }
func (gate *And) GetValue() int  { return gate.sys[gate.in1].GetValue() & gate.sys[gate.in2].GetValue() }
func (gate *Or) GetValue() int   { return gate.sys[gate.in1].GetValue() | gate.sys[gate.in2].GetValue() }
func (gate *Xor) GetValue() int  { return gate.sys[gate.in1].GetValue() ^ gate.sys[gate.in2].GetValue() }

type System map[string]LogicGate

func (s System) Output() int {
	result := 0
	for i := 0; ; i++ {
		key := fmt.Sprintf("z%02d", i)
		gate, ok := s[key]
		if !ok {
			break
		}
		result += gate.GetValue() << i
	}
	return result
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
				sys[match[4]] = &And{match[1], match[3], sys}
			case "OR":
				sys[match[4]] = &Or{match[1], match[3], sys}
			case "XOR":
				sys[match[4]] = &Xor{match[1], match[3], sys}
			default:
				return nil, fmt.Errorf("unrecognized gate type %s", match[2])
			}
		}
	}

	return sys, nil
}

func (sys System) DotGraph() string {
	var sb strings.Builder
	sb.WriteString("digraph {\n")
	for id, gate := range sys {
		switch v := gate.(type) {
		case *Literal:
			var t string
			switch id[0] {
			case 'x', 'y':
				t = "input"
			case 'z':
				t = "output"
			default:
				t = "unknown"
			}

			sb.WriteString(fmt.Sprintf("%s [label=\"%s\n%s\"];\n", id, id, t))
		case *And:
			sb.WriteString(fmt.Sprintf("%s [label=\"%s\nAND\"];\n", id, id))
			sb.WriteString(fmt.Sprintf("%s -> %s;\n", v.in1, id))
			sb.WriteString(fmt.Sprintf("%s -> %s;\n", v.in2, id))
		case *Or:
			sb.WriteString(fmt.Sprintf("%s [label=\"%s\nOR\"];\n", id, id))
			sb.WriteString(fmt.Sprintf("%s -> %s;\n", v.in1, id))
			sb.WriteString(fmt.Sprintf("%s -> %s;\n", v.in2, id))
		case *Xor:
			sb.WriteString(fmt.Sprintf("%s [label=\"%s\nXOR\"];\n", id, id))
			sb.WriteString(fmt.Sprintf("%s -> %s;\n", v.in1, id))
			sb.WriteString(fmt.Sprintf("%s -> %s;\n", v.in2, id))
		}
	}
	sb.WriteRune('}')

	return sb.String()
}

func check(system System) bool {
	maxX := system.MaxValue('x')
	maxY := system.MaxValue('y')
	diffs := make(map[int]int)
	for range 1_000 {
		x := rand.Intn(maxX + 1)
		y := rand.Intn(maxY + 1)

		system.UpdateInput('x', x)
		system.UpdateInput('y', y)

		got := system.Output()
		want := x + y

		diffBits := got ^ want
		for i := 0; diffBits != 0; i++ {
			d := diffBits & 1
			if d == 1 {
				diffs[i]++
			}
			diffBits = diffBits >> 1
		}
	}

	for i := range system.NumBits('z') {
		if diffs[i] != 0 {
			fmt.Printf("Not yet, look around output bit %d\n", i)
			return false
		}
	}

	return true
}
