package pfds

import "testing"

func TestBinHeapEmpty(t *testing.T) {
	h := BinHeap[int]{}
	if !h.Empty() {
		t.Fatal()
	}
	h = h.Push(1)
	if h.Empty() {
		t.Fatal()
	}
}

func TestBinHeapInsert(t *testing.T) {
	h := BinHeap[int]{}
	h = h.Insert(3, 1, 7, 10)
	for _, want := range []int{1, 3, 7, 10} {
		have, ok := h.FindMin()
		if !ok {
			t.Fatal(want)
		}
		if have != want {
			t.Fatalf("wanted %v but have %v", want, have)
		}
		h, ok = h.DeleteMin()
		if !ok {
			t.Fatal()
		}
	}
}

func TestBinHeapMerge(t *testing.T) {
	h1 := BinHeap[int]{}.Insert(3, 1, 7, 10)
	h2 := BinHeap[int]{}.Insert(2, 4, 11, 0)
	h := h1.Merge(h2)
	for _, want := range []int{0, 1, 2, 3, 4, 7, 10, 11} {
		have, ok := h.FindMin()
		if !ok {
			t.Fatal(want)
		}
		if have != want {
			t.Fatalf("wanted %v but have %v", want, have)
		}
		h, ok = h.DeleteMin()
		if !ok {
			t.Fatal()
		}
	}
}
