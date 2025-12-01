package main

import (
	"fmt"
	"log"
	"reflect"
	"testing"
)

var allRules = []string{
	"47|53",
	"97|13",
	"97|61",
	"97|47",
	"75|29",
	"61|13",
	"75|53",
	"29|13",
	"97|29",
	"53|29",
	"61|53",
	"97|53",
	"61|29",
	"47|13",
	"75|47",
	"97|75",
	"47|61",
	"75|61",
	"47|29",
	"75|13",
	"53|13",
}
var ruleset = NewRuleSet()

func init() {
	for _, rule := range allRules {
		r, err := ParseRule(rule)
		if err != nil {
			log.Fatal("main_test.go failed to initialize")
		}

		ruleset.Add(r)
	}
}

var followsRulesTests = []struct {
	seq  []int
	want bool
}{
	{[]int{75, 47, 61, 53, 29}, true},
	{[]int{97, 61, 53, 29, 13}, true},
	{[]int{75, 29, 13}, true},
	{[]int{75, 97, 47, 61, 53}, false},
	{[]int{61, 13, 29}, false},
	{[]int{97, 13, 75, 29, 47}, false},
}

func TestFollowsRules(t *testing.T) {
	for _, testcase := range followsRulesTests {
		name := fmt.Sprintf("%v", testcase.seq)
		t.Run(name, func(t *testing.T) {
			got := ruleset.FollowsRules(testcase.seq)
			if got != testcase.want {
				t.Errorf("got %t, want %t", got, testcase.want)
			}
		})
	}
}

var fixTests = []struct {
	in   []int
	want []int
}{
	{
		in:   []int{75, 97, 47, 61, 53},
		want: []int{97, 75, 47, 61, 53},
	},
	{
		in:   []int{61, 13, 29},
		want: []int{61, 29, 13},
	},
	{
		in:   []int{97, 13, 75, 29, 47},
		want: []int{97, 75, 47, 29, 13},
	},
}

func TestFixSeq(t *testing.T) {
	for _, testcase := range fixTests {
		name := fmt.Sprintf("%v", testcase.in)
		t.Run(name, func(t *testing.T) {
			got := FixSeq(testcase.in, ruleset)
			if !reflect.DeepEqual(testcase.want, got) {
				t.Fatalf("did not get desired fixed sequence.\nwant: %v\ngot:  %v", testcase.want, got)
			}
		})
	}
}
