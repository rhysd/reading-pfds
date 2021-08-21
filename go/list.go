package pfds

import (
	"fmt"
	"strings"
)

type List[T any] struct {
	val T
	next *List[T]
}

func NewList[T any]() *List[T] {
	return nil
}

func (l *List[T]) Empty() bool {
	return l == nil
}

func (l *List[T]) Cons(v T) *List[T] {
	return &List[T]{v, l}
}

func (l *List[T]) Head() (T, bool) {
	if l == nil {
		var zero T
		return zero, false
	}
	return l.val, true
}

func (l *List[T]) Tail() (*List[T], bool) {
	if l == nil {
		return nil, false
	}
	return l.next, true
}

func (l *List[T]) Uncons() (T, *List[T], bool) {
	if l == nil {
		var zero T
		return zero, nil, false
	}
	return l.val, l.next, true
}

func (l *List[T]) Concat(other *List[T]) *List[T] {
	if l == nil {
		return other
	}
	return l.next.Concat(other).Cons(l.val)
}

func (l *List[T]) Update(idx int, x T) (*List[T], bool) {
	if l == nil {
		return nil, false
	}
	if idx == 0 {
		return &List[T]{x, l.next}, true
	}
	l2, ok := l.next.Update(idx-1, x)
	if !ok {
		return nil, false
	}
	return l2.Cons(l.val), true
}

func (l *List[T]) rev(ret *List[T]) *List[T] {
	if l == nil {
		return ret
	}
	return l.next.rev(ret.Cons(l.val))
}

func (l *List[T]) Rev() *List[T] {
	return l.rev(nil)
}

func (l *List[T]) Len() int {
	if l == nil {
		return 0
	}
	return l.next.Len() + 1
}

func (l *List[T]) buildSlice(s []T) []T {
	if l == nil {
		return s
	}
	return l.next.buildSlice(append(s, l.val))
}

func (l *List[T]) ToSlice() []T {
	return l.buildSlice(nil)
}

func (l *List[T]) buildString(b *strings.Builder) {
	if l == nil {
		b.WriteRune(']')
		return
	}
	b.WriteString(fmt.Sprintf("%v", l.val))
	if l.next != nil {
		b.WriteRune(' ')
	}
	l.buildString(b)
}

func (l *List[T]) String() string {
	var b strings.Builder
	b.WriteRune('[')
	l.buildString(&b)
	return b.String()
}
