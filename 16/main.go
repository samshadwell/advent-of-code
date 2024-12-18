package main

import (
	"container/heap"
	"errors"
	"fmt"
	"log"
	"os"

	"advent-of-code/util/grids"
)

var directions = []grids.Location{
	{Row: 0, Col: 1},  // East
	{Row: 1, Col: 0},  // South
	{Row: 0, Col: -1}, // West
	{Row: -1, Col: 0}, // North
}

type workItem struct {
	location  grids.Location
	parent    grids.Location
	direction int
	cost      int
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

func findBestPath(maze [][]rune) (int, int, error) {
	start, ok := grids.FindRune(maze, 'S')
	if !ok {
		return 0, 0, errors.New("did not find start location in maze")
	}

	end, ok := grids.FindRune(maze, 'E')
	if !ok {
		return 0, 0, errors.New("did not find end location in maze")
	}

	h := &workHeap{workItem{start, start, 0, 0}}
	heap.Init(h)

	lowestCosts := make(map[grids.Location]int)
	parents := make(map[grids.Location][]grids.Location)

	for h.Len() > 0 {
		wi := heap.Pop(h).(workItem)

		if wi.location == (grids.Location{Row: 7, Col: 5}) {
			fmt.Printf("%v\n", wi)
		}

		if prevBest, ok := lowestCosts[wi.location]; ok {
			if prevBest == wi.cost {
				parents[wi.location] = append(parents[wi.location], wi.parent)
			}
			continue
		}
		lowestCosts[wi.location] = wi.cost
		parents[wi.location] = []grids.Location{wi.parent}

		straight := wi.location.Plus(directions[wi.direction])
		if maze[straight.Row][straight.Col] != '#' {
			heap.Push(h, workItem{
				straight,
				wi.location,
				wi.direction,
				wi.cost + 1,
			})
		}

		rightDir := (wi.direction + 1) % 4
		right := wi.location.Plus(directions[rightDir])
		if maze[right.Row][right.Col] != '#' {
			heap.Push(h, workItem{
				right,
				wi.location,
				rightDir,
				wi.cost + 1001,
			})
		}

		leftDir := (wi.direction + 3) % 4
		left := wi.location.Plus(directions[leftDir])
		if maze[left.Row][left.Col] != '#' {
			heap.Push(h, workItem{
				left,
				wi.location,
				leftDir,
				wi.cost + 1001,
			})
		}
	}

	cost, ok := lowestCosts[end]
	if !ok {
		return 0, 0, errors.New("did not find path to end")
	}

	onPaths := make(map[grids.Location]bool)
	queue := []grids.Location{end}
	for len(queue) > 0 {
		l := queue[len(queue)-1]
		queue = queue[:len(queue)-1]
		onPaths[l] = true
		if l != start {
			queue = append(queue, parents[l]...)
		}
	}

	return cost, len(onPaths), nil
}
