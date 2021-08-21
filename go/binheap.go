package pfds

type LessEq[T any] interface {
	LessEq(T) bool
}

type binHeapTree[T LessEq[T]] struct {
	rank int
	elem T
	children *List[*binHeapTree[T]]
}

func (t *binHeapTree[T]) link(other *binHeapTree[T]) *binHeapTree[T] {
	if t.elem.LessEq(other.elem) {
		return &binHeapTree[T]{t.rank + 1, t.elem, t.children.Cons(other)}
	}
	return &binHeapTree[T]{t.rank + 1, other.elem, other.children.Cons(t)}
}

func (t *binHeapTree[T]) insertTo(ts *List[*binHeapTree[T]]) *List[*binHeapTree[T]] {
	t2, ts2, ok := ts.Uncons()
	if !ok {
		// empty
		return ts.Cons(t)
	}

	if t.rank < t2.rank {
		return ts.Cons(t)
	}

	return t.link(t2).insertTo(ts2)
}

func mergeBinHeapTrees[T LessEq[T]](lhs, rhs *List[*binHeapTree[T]]) *List[*binHeapTree[T]] {
	t1, ts1, ok := lhs.Uncons()
	if !ok {
		return rhs
	}
	t2, ts2, ok := rhs.Uncons()
	if !ok {
		return lhs
	}
	if t1.rank < t2.rank {
		return mergeBinHeapTrees(ts1, rhs).Cons(t1)
	}
	if t2.rank < t1.rank {
		return mergeBinHeapTrees(lhs, ts2).Cons(t2)
	}
	return t1.link(t2).insertTo(mergeBinHeapTrees(ts1, ts2))
}

func removeMinBinHeapTrees[T LessEq[T]](trees *List[*binHeapTree[T]]) (*binHeapTree[T], *List[*binHeapTree[T]], bool) {
	t, ts, ok := trees.Uncons()
	if !ok {
		return nil, nil, false
	}
	if ts == nil {
		return t, nil, true
	}
	t2, ts2, _ := removeMinBinHeapTrees(ts)
	if t.elem.LessEq(t2.elem) {
		return t, ts, true
	}
	return t2, ts2.Cons(t), true
}

type BinHeap[T LessEq[T]] struct {
	trees *List[*binHeapTree[T]]
}

func (h BinHeap[T]) Empty() bool {
	return h.trees.Empty()
}

func (h BinHeap[T]) Insert(xs... T) BinHeap[T] {
	ts := h.trees
	for _, x := range xs {
		n := &binHeapTree[T]{0, x, nil}
		ts = n.insertTo(ts)
	}
	return BinHeap[T]{ts}
}

func (h BinHeap[T]) Push(x T) BinHeap[T] {
	n := &binHeapTree[T]{0, x, nil}
	return BinHeap[T]{n.insertTo(h.trees)}
}

func (h BinHeap[T]) Merge(other BinHeap[T]) BinHeap[T] {
	return BinHeap[T]{mergeBinHeapTrees(h.trees, other.trees)}
}

func (h BinHeap[T]) FindMin() (T, bool) {
	t, _, ok := removeMinBinHeapTrees(h.trees)
	if !ok {
		var zero T
		return zero, false
	}
	return t.elem, true
}

func (h BinHeap[T]) DeleteMin() (BinHeap[T], bool) {
	t, ts, ok := removeMinBinHeapTrees(h.trees)
	if !ok {
		return BinHeap[T]{}, false
	}
	ts2 := mergeBinHeapTrees(t.children.Rev(), ts)
	return BinHeap[T]{ts2}, true
}
