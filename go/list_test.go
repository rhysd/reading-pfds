package pfds

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestListEmpty(t *testing.T) {
	l := NewList[int]()
	l2 := l.Cons(1)
	l3 := l2.Cons(2)

	if !l.Empty() {
		t.Fatal("not empty", l)
	}
	if l2.Empty() {
		t.Fatal("empty", l2)
	}
	if l3.Empty() {
		t.Fatal("empty", l3)
	}
}

func TestListHead(t *testing.T) {
	l := NewList[int]()
	l2 := l.Cons(1)
	l3 := l2.Cons(2)

	if _, ok := l.Head(); ok {
		t.Fatal(l)
	}
	if h, ok := l2.Head(); !ok || h != 1 {
		t.Fatal(ok, h)
	}
	if h, ok := l3.Head(); !ok || h != 2 {
		t.Fatal(ok, h)
	}
}

func TestListTail(t *testing.T) {
	l := NewList[int]()
	l2 := l.Cons(1)
	l3 := l2.Cons(2)
	l4, ok := l3.Tail()
	if !ok {
		t.Fatal(l3)
	}
	l5, ok := l2.Tail()
	if !ok {
		t.Fatal(l2)
	}
	if l4 != l2 {
		t.Fatal(l2, l4)
	}
	if l5 != l {
		t.Fatal(l, l5)
	}
}

func TestListConcat(t *testing.T) {
	l := NewList[int]()
	l2 := l.Cons(1)
	l3 := l2.Cons(2)
	l4 := l3.Cons(3)
	l5 := l4.Concat(l3).Concat(l2)

	want := []int{3, 2, 1, 2, 1, 1}
	if s := l5.ToSlice(); !cmp.Equal(s, want) {
		t.Fatal(cmp.Diff(s, want))
	}
}

func TestListUpdate(t *testing.T) {
	l := NewList[int]().Cons(1).Cons(2).Cons(3)

	l2, ok := l.Update(0, 10)
	if !ok {
		t.Fatal(l)
	}
	if !cmp.Equal(l2.ToSlice(), []int{10, 2, 1}) {
		t.Fatal(l2)
	}
	if !cmp.Equal(l.ToSlice(), []int{3, 2, 1}) {
		t.Fatal(l)
	}

	l3, ok := l.Update(2, 10)
	if !ok {
		t.Fatal(l)
	}
	if !cmp.Equal(l3.ToSlice(), []int{3, 2, 10}) {
		t.Fatal(l3)
	}
	if !cmp.Equal(l.ToSlice(), []int{3, 2, 1}) {
		t.Fatal(l)
	}

	_, ok = l.Update(100, 10)
	if ok {
		t.Fatal(l)
	}
}
