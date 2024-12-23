package main

import (
	"sort"
	"testing"

	"github.com/google/go-cmp/cmp"
)

var edges = [][]string{
	{"kh", "tc"},
	{"qp", "kh"},
	{"de", "cg"},
	{"ka", "co"},
	{"yn", "aq"},
	{"qp", "ub"},
	{"cg", "tb"},
	{"vc", "aq"},
	{"tb", "ka"},
	{"wh", "tc"},
	{"yn", "cg"},
	{"kh", "ub"},
	{"ta", "co"},
	{"de", "co"},
	{"tc", "td"},
	{"tb", "wq"},
	{"wh", "td"},
	{"ta", "ka"},
	{"td", "qp"},
	{"aq", "cg"},
	{"wq", "ub"},
	{"ub", "vc"},
	{"de", "ta"},
	{"wq", "aq"},
	{"wq", "vc"},
	{"wh", "yn"},
	{"ka", "de"},
	{"kh", "ta"},
	{"co", "tc"},
	{"wh", "qp"},
	{"tb", "vc"},
	{"td", "yn"},
}

var graph = make(Graph)

func init() {
	for _, e := range edges {
		n1, n2 := e[0], e[1]
		graph.AddUndirectedEdge(n1, n2)
	}
}

func TestFindConnectedTriples(t *testing.T) {
	want := []triple{
		{"co", "de", "ta"},
		{"co", "ka", "ta"},
		{"de", "ka", "ta"},
		{"qp", "td", "wh"},
		{"tb", "vc", "wq"},
		{"tc", "td", "wh"},
		{"td", "wh", "yn"},
	}

	got := findConnectedTriples(graph)
	if len(got) != len(want) {
		t.Fatalf("got length did not match want length. len(got): %d, len(want): %d", len(got), len(want))
	}

	for _, trip := range want {
		if !got[trip] {
			t.Fatalf("got did not have all expected triples, missing: %v", trip)
		}
	}
}

func TestMaximallyGrowClique(t *testing.T) {
	want := []string{"co", "de", "ka", "ta"}
	got := maximallyGrowClique(graph, []string{}, graph.AllNodes())
	sort.Strings(got)
	if diff := cmp.Diff(want, got); diff != "" {
		t.Fatalf("clique does not match expected. Diff (-want +got):\n%s", diff)
	}
}
