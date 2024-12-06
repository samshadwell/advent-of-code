package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"slices"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("failed to open file: %v", err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	ruleBlock := true

	allRules := NewRuleSet()
	part1 := 0
	part2 := 0

	for scanner.Scan() {
		nextLine := scanner.Text()
		if nextLine == "" {
			ruleBlock = false
			continue
		}

		if ruleBlock {
			nextRule, err := ParseRule(nextLine)
			if err != nil {
				log.Fatalf("error parsing rule: %v", err)
			}

			allRules.Add(nextRule)
		} else {
			stringSeq := strings.Split(nextLine, ",")
			seq := make([]int, 0, len(stringSeq))
			for _, s := range stringSeq {
				n, err := strconv.Atoi(s)
				if err != nil {
					log.Fatalf("error parsing update: %v", err)
				}
				seq = append(seq, n)
			}

			if allRules.FollowsRules(seq) {
				part1 += seq[len(seq)/2]
			} else {
				fixed := FixSeq(seq, allRules)
				part2 += fixed[len(fixed)/2]
			}
		}
	}

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}

type Rule struct {
	before int
	after  int
}

type RuleSet struct {
	rules map[ruleKey]Rule
}

type ruleKey struct {
	low  int
	high int
}

var ruleRegex = regexp.MustCompile(`(\d+)\|(\d+)`)

func ParseRule(input string) (Rule, error) {
	match := ruleRegex.FindStringSubmatch(input)
	if match == nil {
		return Rule{}, fmt.Errorf("unable to parse rule '%s'", input)
	}

	if len(match) != 3 {
		return Rule{}, fmt.Errorf("programmer error, ParseRule regex match should have exactly 3 elements, got %d", len(match))
	}

	before, err := strconv.Atoi(match[1])
	if err != nil {
		return Rule{}, fmt.Errorf("error while parsing left side of rule to int. Input: '%s', err: %w", input, err)
	}

	after, err := strconv.Atoi(match[2])
	if err != nil {
		return Rule{}, fmt.Errorf("error while parsing right side of rule to int. Input: '%s', err: %w", input, err)
	}

	return Rule{
		before,
		after,
	}, nil
}

func NewRuleSet() RuleSet {
	return RuleSet{
		rules: make(map[ruleKey]Rule),
	}
}

func (r *RuleSet) Add(rule Rule) {
	key := ruleKey{
		low:  min(rule.before, rule.after),
		high: max(rule.before, rule.after),
	}

	r.rules[key] = rule
}

func (r *RuleSet) FollowsRules(seq []int) bool {
	for i, first := range seq {
		for _, second := range seq[i+1:] {
			key := ruleKey{
				low:  min(first, second),
				high: max(first, second),
			}

			rule, ok := r.rules[key]
			if !ok {
				continue
			}

			if first != rule.before {
				return false
			}
		}
	}

	return true
}

func FixSeq(seq []int, rules RuleSet) []int {
	relevantRules := make([]Rule, 0)
	for i, first := range seq {
		for _, second := range seq[i+1:] {
			key := ruleKey{
				low:  min(first, second),
				high: max(first, second),
			}

			rule, ok := rules.rules[key]
			if !ok {
				continue
			}

			relevantRules = append(relevantRules, rule)
		}
	}

	// Edges stored as to -> from, the reverse order of a "normal" adjacency list
	// Invariant: All values have at least one element
	revEdges := make(map[int][]int)
	for _, rule := range relevantRules {
		_, ok := revEdges[rule.after]
		if !ok {
			revEdges[rule.after] = make([]int, 0, 1)
		}

		revEdges[rule.after] = append(revEdges[rule.after], rule.before)
	}

	// Kahn's algorithm, from https://en.wikipedia.org/wiki/Topological_sorting
	l := make([]int, 0, len(seq))
	s := make([]int, 0, len(seq))
	for _, item := range seq {
		befores := revEdges[item]
		if befores == nil {
			s = append(s, item)
		}
	}

	for len(s) > 0 {
		n := s[len(s)-1]
		s = s[:len(s)-1]

		l = append(l, n)
		for m, edges := range revEdges {
			idx := slices.Index(edges, n)
			if idx >= 0 {
				newEdges := slices.Delete(edges, idx, idx+1)
				if len(newEdges) == 0 {
					s = append(s, m)
					delete(revEdges, m)
				} else {
					revEdges[m] = newEdges
				}
			}
		}
	}
	return l
}
