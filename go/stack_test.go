package pfds

import "testing"

func TestStackNew(t *testing.T) {
	s := Stack[int]{}
	if len(s) != 0 {
		t.Fatal("stack is not empty", s)
	}
}

func TestStackPushPop(t *testing.T) {
	s := Stack[int]{}
	s = s.Push(1)
	s = s.Push(2)
	s = s.Push(3)

	var h int
	var ok bool
	for i, w := range []int{3, 2, 1} {
		s, h, ok = s.Pop()
		if !ok {
			t.Fatal("stack is empty at", i)
		}
		if w != h {
			t.Fatalf("wanted %v but have %v", w, h)
		}
	}

	if len(s) != 0 {
		t.Fatal("stack is not empty", s)
	}
}

func TestStackTop(t *testing.T) {
	s := Stack[int]{}
	s = s.Push(1)

	x, ok := s.Top()
	if !ok {
		t.Fatal("stack is empty")
	}
	if x != 1 {
		t.Fatalf("wanted 1 but have %v", x)
	}
}

func TestStackEmptyPop(t *testing.T) {
	s := Stack[int]{}
	s, _, ok := s.Pop()
	if ok {
		t.Fatal("pop was unexpectedly successful")
	}
	if len(s) != 0 {
		t.Fatal("stack is not empty")
	}
}

func TestStackImmutable(t *testing.T) {
	s1 := Stack[int]{}
	s2 := s1.Push(1)
	s3 := s1.Push(2)
	if len(s1) != 0 || len(s2) != 1 || len(s3) != 1 {
		t.Fatalf("%d %d %d", len(s1), len(s2), len(s3))
	}
}
