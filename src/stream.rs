// p.41 - p.44
//
// Stream: Lazyily evaluated list

use std::fmt::Debug;
use lazy::Delayed;

#[derive(Debug, Clone)]
enum StreamCell<'a, T: 'a + Clone + Debug> {
    Nil,
    Cons(T, Cell<'a, T>),
}

type Cell<'a, T> = Delayed<'a, StreamCell<'a, T>>;

use self::StreamCell::*;

#[derive(Debug, Clone)]
pub struct Stream<'a, T: 'a + Clone + Debug>(Cell<'a, T>);

impl<'a, T> Stream<'a, T>
where T: 'a + Clone + Debug {
    pub fn cons(&'a self, d: &'a Delayed<'a, T>) -> Self {
        Stream(lazily!{
            Cons(d.eval().clone(), self.0.clone())
        })
    }

    fn concat_impl(lhs: &'a Cell<'a, T>, rhs: &'a Cell<'a, T>) -> Cell<'a, T> {
        match *lhs.eval() {
            Nil => rhs.clone(),
            Cons(ref x, ref xs) => lazily!{
                Cons(x.clone(), Stream::concat_impl(xs, rhs))
            },
        }
    }
    pub fn concat(&'a self, other: &'a Self) -> Self {
        Stream(Stream::concat_impl(&self.0, &other.0))
    }

    fn take_impl(s: &'a Cell<'a, T>, u: usize) -> Cell<'a, T> {
        if u == 0 {
            return s.clone();
        }
        match *s.eval() {
            Nil => Delayed::constant(Nil),
            Cons(ref x, ref xs) => lazily!{
                Cons(x.clone(), Stream::take_impl(xs, u-1))
            },
        }
    }
    pub fn take(&'a self, u: usize) -> Self {
        Stream(Stream::take_impl(&self.0, u))
    }

    fn drop_impl(xs: &'a Cell<'a, T>, u: usize) -> Cell<'a, T> {
        if u == 0 {
            return xs.clone();
        }
        match *xs.eval() {
            Nil => Delayed::constant(Nil),
            Cons(_, ref ys) => Stream::drop_impl(ys, u-1),
        }
    }
    pub fn drop(&'a self, u: usize) -> Self {
        Stream(lazily!{
            Stream::drop_impl(&self.0, u).eval().clone()
        })
    }

    fn reverse_impl(acc: Cell<'a, T>, xs: &'a Cell<'a, T>) -> Cell<'a, T> {
        match *xs.eval() {
            Nil => acc,
            Cons(ref y, ref ys) => Stream::reverse_impl(lazily!{
                Cons(y.clone(), acc.clone()/*Cannot move variables out of the captured lambda...*/)
            }, ys),
        }
    }
    pub fn reverse(&'a self) -> Self {
        Stream(lazily!{
            Stream::reverse_impl(Delayed::constant(Nil), &self.0).eval().clone()
        })
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        // TODO
    }
} // mod tests
