// p.17 - p.21

use std::fmt::Debug;
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
pub enum List<T>
    where T: Clone + PartialEq + Debug {
    Nil,
    Cons(T, Rc<List<T>>),
}

impl<T> List<T>
    where T: Clone + PartialEq + Debug {

    pub fn empty() -> Self {
        List::Nil
    }

    pub fn is_empty(&self) -> bool {
        *self == List::Nil
    }

    pub fn cons(&self, v: T) -> Self {
        List::Cons(v, Rc::new(self.clone()))
    }

    pub fn head(&self) -> &T {
        match *self {
            List::Nil => panic!("List is empty!"),
            List::Cons(ref x, _) => x,
        }
    }

    pub fn tail(&self) -> &Self {
        match *self {
            List::Nil => panic!("List is empty!"),
            List::Cons(_, ref xs) => xs,
        }
    }

    pub fn concat(&self, rhs: List<T>) -> Self {
        match *self {
            List::Nil => rhs,
            List::Cons(ref x, ref xs) => xs.concat(rhs).cons(x.clone()),
        }
    }

    pub fn update_at(&self, idx: u32, v: T) -> Self {
        match *self {
            List::Nil => panic!("List is empty!"),
            List::Cons(ref x, ref xs) =>
                if idx == 0 {
                    List::Cons(v, xs.clone())
                } else {
                    xs.update_at(idx - 1, v).cons(x.clone())
                },
        }
    }

    pub fn suffixes(&self) -> List<Self> {
        match *self {
            List::Nil => List::Nil.cons(List::Nil),
            List::Cons(_, ref xs) => xs.suffixes().cons(self.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let e = List::empty();
        assert!(e.is_empty());

        let e = e.cons(1);
        assert_eq!(e.head(), &1);
        let e = e.cons(2);
        assert_eq!(e.head(), &2);
        assert_eq!(e.tail().head(), &1);

        let e2 = List::empty().cons(3).cons(4);
        let e = e.concat(e2);
        assert_eq!(e.head(), &2);
        assert_eq!(e.tail().tail().head(), &4);

        let e2 = e.update_at(1, 5);
        assert_eq!(e2.tail().head(), &5);

        let e = e.suffixes();
        assert_eq!(e.tail().head().tail().head(), &4);
    }
}
