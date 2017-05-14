// p.17 - p.21

use std::fmt::Debug;
use std::rc::Rc;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub enum Node<T: Clone + Debug> {
    Nil,
    Cons(T, Link<T>),
}

type Link<T> = Rc<Node<T>>;

// Just a wrapper of Rc<Node<T>> because `impl` cannot be used for external type `Rc`.
// Below cannot compile.
//
// type List<T> = Rc<Node<T>>;
// impl<T> List<T> { ... }
#[derive(Clone, Debug)]
pub struct List<T: Clone + Debug>(Link<T>);

fn list<T: Clone + Debug>(n: Node<T>) -> List<T> {
    List(Rc::new(n))
}

impl<T> List<T>
where T: Clone + Debug {

    pub fn empty() -> Self {
        list(Node::Nil)
    }

    pub fn is_empty(&self) -> bool {
        match *self.0 {
            Node::Nil => true,
            _ => false,
        }
    }

    pub fn cons(&self, v: T) -> Self {
        list(Node::Cons(v, self.0.clone()))
    }

    pub fn head(&self) -> &T {
        match *self.0 {
            Node::Nil => panic!("Node is empty!"),
            Node::Cons(ref x, _) => x,
        }
    }

    pub fn tail(&self) -> Self {
        match *self.0 {
            Node::Nil => panic!("Node is empty!"),
            Node::Cons(_, ref xs) => List(xs.clone()),
        }
    }

    fn concat_impl(n: &Link<T>, other: &Self) -> Self {
        match **n {
            Node::Nil => other.clone(),
            Node::Cons(ref x, ref xs) => List::concat_impl(xs, other).cons(x.clone()),
        }
    }

    // Preceding list can be shared and don't need to be cloned.
    pub fn concat(&self, other: &List<T>) -> Self {
        List::concat_impl(&self.0, other)
    }

    fn update_at_impl(n: &Link<T>, idx: u32, v: T) -> Self {
        match **n {
            Node::Nil => panic!("Node is empty!"),
            Node::Cons(ref x, ref xs) =>
                if idx == 0 {
                    list(Node::Cons(v, xs.clone()))
                } else {
                    List::update_at_impl(n, idx - 1, v).cons(x.clone())
                },
        }
    }

    // Cells after `idx` can be shared (don't need to copy)
    pub fn update_at(&self, idx: u32, v: T) -> Self {
        List::update_at_impl(&self.0, idx, v)
    }

    fn suffixes_impl(n: &Link<T>) -> List<Self> {
        match **n {
            Node::Nil => List::empty().cons(List::empty()),
            Node::Cons(_, ref xs) => List::suffixes_impl(xs).cons(List(n.clone())),
        }
    }

    // Elements of returned list are shared.
    pub fn suffixes(&self) -> List<Self> {
        List::suffixes_impl(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let e = List::empty();
        assert!(e.is_empty());

        // 1
        let e = e.cons(1);
        assert_eq!(e.head(), &1);
        // 2, 1
        let e = e.cons(2);
        assert_eq!(e.head(), &2);
        assert_eq!(e.tail().head(), &1);

        // 4, 3
        let e2 = List::empty().cons(3).cons(4);
        // 2, 1, 4, 3
        let e = e.concat(&e2);
        assert_eq!(e.head(), &2);
        assert_eq!(e.tail().tail().head(), &4);

        // 2, 5, 4, 3
        let e2 = e.update_at(1, 5);
        assert_eq!(e2.tail().head(), &5);

        let e = e.suffixes();
        assert_eq!(e.tail().head().tail().head(), &4);
    }
}
