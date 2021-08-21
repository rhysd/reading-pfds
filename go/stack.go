package pfds

type Stack[T any] []T

func (s Stack[T]) Push(v T) Stack[T] {
	return append(s, v)
}

func (s Stack[T]) Pop() (Stack[T], T, bool) {
	l := len(s)
	if l == 0 {
		var zero T // https://go.googlesource.com/proposal/+/refs/heads/master/design/43651-type-parameters.md#the-zero-value
		return s, zero, false
	}
	return s[:l-1], s[l-1], true
}

func (s Stack[T]) Top() (T, bool) {
	l := len(s)
	if l == 0 {
		var zero T
		return zero, false
	}
	return s[l-1], true
}
