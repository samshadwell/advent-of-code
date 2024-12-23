package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"slices"
	"sort"
	"strings"
)

type Set map[string]bool

func (s Set) Intersection(other Set) Set {
	res := make(Set)
	for e := range s {
		if _, ok := other[e]; ok {
			res[e] = true
		}
	}
	return res
}

type Graph map[string]Set

func (g Graph) AddUndirectedEdge(node1, node2 string) {
	g.AddEdge(node1, node2)
	g.AddEdge(node2, node1)
}

func (g Graph) AddEdge(from, to string) {
	if _, ok := g[from]; !ok {
		g[from] = make(Set)
	}
	g[from][to] = true
}

func (g Graph) AllNodes() []string {
	nodes := make([]string, 0, len(g))
	for n := range g {
		nodes = append(nodes, n)
	}
	return nodes
}

type triple struct {
	n1, n2, n3 string
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening input file: %v", err)
	}

	g := make(Graph)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		split := strings.Split(line, "-")
		if len(split) != 2 {
			log.Fatalf("error parsing input file, line: %s", line)
		}

		n1, n2 := split[0], split[1]
		g.AddUndirectedEdge(n1, n2)
	}

	triples := findPart1Triples(g)
	part1 := len(triples)

	maxClique := maximallyGrowClique(g, []string{}, g.AllNodes())
	sort.Strings(maxClique)
	part2 := strings.Join(maxClique, ",")

	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %s\n", part2)
}

func findPart1Triples(g Graph) map[triple]bool {
	triples := make(map[triple]bool)
	for n1, neighbors := range g {
		if !strings.HasPrefix(n1, "t") {
			continue
		}

		for n2 := range neighbors {
			common := neighbors.Intersection(g[n2])
			for n3 := range common {
				nodes := []string{n1, n2, n3}
				sort.Strings(nodes)
				triples[triple{nodes[0], nodes[1], nodes[2]}] = true
			}
		}
	}
	return triples
}

func maximallyGrowClique(g Graph, clique []string, candidates []string) []string {
	bestLen := len(clique)
	best := clique
	originalLength := len(clique)

	for i, candidate := range candidates {
		if canAdd(g, candidate, clique) {
			clique := append(clique, candidate)
			// To avoid adding the same things in multiple orders, only consider adding nodes from i -> end
			// from now on. In go, this still points to the same underlying memory as `candidates`
			newCandidates := candidates[i+1:]
			clique = maximallyGrowClique(g, clique, newCandidates)
			if len(clique) > bestLen {
				best = slices.Clone(clique)
				bestLen = len(best)
			}
		}
		clique = clique[:originalLength]
	}

	// Copy the best back to the original clique
	clique = clique[:originalLength]
	for i := originalLength; i < len(best); i++ {
		clique = append(clique, best[i])
	}

	return clique
}

func canAdd(g Graph, candidate string, clique []string) bool {
	candidateNeighbors := g[candidate]
	for _, existing := range clique {
		if candidate == existing {
			return false
		}

		if !candidateNeighbors[existing] {
			return false
		}
	}
	return true
}
