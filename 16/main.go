package main

import (
	"container/heap"
	"errors"
	"fmt"
	"log"
	"math"
	"os"

	"advent-of-code/util/grids"
)

var directions = []grids.Location{
	grids.Right(),
	grids.Down(),
	grids.Left(),
	grids.Up(),
}

// A _loc_ation, _dir_ection tuple. Useful since we care not only about where a given
// reindeer is on the map, but also what direction it is facing
type locDir struct {
	location  grids.Location
	direction int
}

type workItem struct {
	current locDir
	parent  *locDir
	cost    int
}

// See https://pkg.go.dev/container/heap, particularly the IntHeap example
type workHeap []workItem

func (h workHeap) Len() int           { return len(h) }
func (h workHeap) Less(i, j int) bool { return h[i].cost < h[j].cost }
func (h workHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *workHeap) Push(x any) {
	*h = append(*h, x.(workItem))
}

func (h *workHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatalf("error while opening file: %v", err)
	}
	defer file.Close()

	maze := grids.ParseRuneGrid(file)
	part1, part2, err := findBestPath(maze)
	if err != nil {
		log.Fatalf("error finding cheapest path: %v", err)
	}
	fmt.Printf("Part 1: %d\n", part1)
	fmt.Printf("Part 2: %d\n", part2)
}

const (
	straightCost = 1
	turnCost     = 1000
)

func findBestPath(maze [][]rune) (int, int, error) {
	start, ok := grids.FindRune(maze, 'S')
	if !ok {
		return 0, 0, errors.New("did not find start location in maze")
	}

	end, ok := grids.FindRune(maze, 'E')
	if !ok {
		return 0, 0, errors.New("did not find end location in maze")
	}

	h := &workHeap{workItem{locDir{start, 0}, nil, 0}}
	heap.Init(h)

	// Map from a given (location, direction) tuple to the lowest cost required to get there
	lowestCosts := make(map[locDir]int)
	// The parent (location, direction) tuples along all lowest-cost paths to a given (location, direction)
	parents := make(map[locDir][]*locDir)

	for h.Len() > 0 {
		wi := heap.Pop(h).(workItem)
		if prevBest, ok := lowestCosts[wi.current]; ok && prevBest < wi.cost {
			// If we've already found a better route to this locDir do nothing
			continue
		} else if ok && prevBest == wi.cost {
			// We've found an equally-good route to this locDir, update the parents
			parents[wi.current] = append(parents[wi.current], wi.parent)
			continue
		}
		// Implicit else: this is the best route we've found to this locDir. We will find
		// no better routes, as the costs will monotonically increase from here
		lowestCosts[wi.current] = wi.cost
		parents[wi.current] = []*locDir{wi.parent}

		// From here we can do three things: go straight, turn left, or turn right
		straight := wi.current.location.Plus(directions[wi.current.direction])
		if maze[straight.Row][straight.Col] != '#' {
			heap.Push(h, workItem{
				locDir{straight, wi.current.direction},
				&wi.current,
				wi.cost + straightCost,
			})
		}

		rightDir := (wi.current.direction + 1) % 4
		heap.Push(h, workItem{
			locDir{wi.current.location, rightDir},
			&wi.current,
			wi.cost + turnCost,
		})

		leftDir := (wi.current.direction + 3) % 4
		heap.Push(h, workItem{
			locDir{wi.current.location, leftDir},
			&wi.current,
			wi.cost + turnCost,
		})
	}

	// Because we don't care about the orientation at which we reach the end, find the
	// lowest-cost among all the directions
	minCost := math.MaxInt
	minCosts := make([]locDir, 0)
	for d := 0; d < len(directions); d++ {
		ld := locDir{end, d}
		if c, ok := lowestCosts[ld]; ok && c < minCost {
			minCost = c
			minCosts = []locDir{ld}
		} else if ok && c == minCost {
			minCosts = append(minCosts, ld)
		}
	}

	if len(minCosts) == 0 {
		return 0, 0, errors.New("did not find path to end")
	}

	part1 := minCost
	part2 := part2(minCosts, parents)

	return part1, part2, nil
}

func part2(minCosts []locDir, parents map[locDir][]*locDir) int {
	locations := make(map[grids.Location]bool)
	queue := make([]*locDir, 0)
	for _, ld := range minCosts {
		queue = append(queue, &ld)
	}
	for len(queue) > 0 {
		l := queue[len(queue)-1]
		queue = queue[:len(queue)-1]

		if l == nil {
			continue
		}

		locations[l.location] = true
		ps := parents[*l]
		queue = append(queue, ps...)
	}
	return len(locations)
}
