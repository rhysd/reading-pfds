// p.17 - p.21

use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Node<T: Clone + Debug> {
    Nil,
    Cons(T, List<T>),
}

use self::Node::*;

// Just a wrapper of Rc<Node<T>> because `impl` cannot be used for external type `Rc`.
#[derive(Clone, Debug)]
pub struct List<T: Clone + Debug>(pub Rc<Node<T>>);

fn list<T: Clone + Debug>(n: Node<T>) -> List<T> {
    List(Rc::new(n))
}

impl<T> List<T>
where T: Clone + Debug {

    pub fn empty() -> Self {
        list(Nil)
    }

    pub fn one(v: T) -> Self {
        list(Cons(v, list(Nil)))
    }

    pub fn is_empty(&self) -> bool {
        match *self.0 {
            Nil => true,
            _ => false,
        }
    }

    pub fn cons(&self, v: T) -> Self {
        list(Cons(v, self.clone()))
    }

    pub fn root(&self) -> &Node<T> {
        &*self.0
    }

    pub fn head(&self) -> &T {
        match *self.0 {
            Nil => panic!("Node is empty!"),
            Cons(ref x, _) => x,
        }
    }

    pub fn tail(&self) -> &Self {
        match *self.0 {
            Nil => panic!("Node is empty!"),
            Cons(_, ref xs) => xs,
        }
    }

    // Preceding list can be shared and don't need to be cloned.
    pub fn concat(&self, other: &List<T>) -> Self {
        match *self.0 {
            Nil => other.clone(),
            Cons(ref x, ref xs) => xs.concat(other).cons(x.clone()),
        }
    }

    // Cells after `idx` can be shared (don't need to copy)
    pub fn update_at(&self, idx: u32, v: T) -> Self {
        match *self.0 {
            Nil => panic!("Node is empty!"),
            Cons(ref x, ref xs) =>
                if idx == 0 {
                    list(Cons(v, xs.clone()))
                } else {
                    xs.update_at(idx-1, v).cons(x.clone())
                },
        }
    }

    // Elements of returned list are shared.
    pub fn suffixes(&self) -> List<Self> {
        match *self.0 {
            Nil => List::one(List::empty()),
            Cons(_, ref xs) => xs.suffixes().cons(self.clone()),
        }
    }

    fn rev_impl(&self, ret: Self) -> Self {
        match *self.0 {
            Nil => ret,
            Cons(ref x, ref xs) => xs.rev_impl(ret.cons(x.clone()))
        }
    }

    pub fn rev(&self) -> Self {
        self.rev_impl(List::empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let e = List::empty();
        assert!(e.is_empty());
        let e = e.cons(1);
        assert!(!e.is_empty());
    }

    #[test]
    fn test_cons() {
        // 1
        let e = List::one(1);
        assert_eq!(e.head(), &1);
        // 2, 1
        let e = e.cons(2);
        assert_eq!(e.head(), &2);
        assert_eq!(e.tail().head(), &1);
    }

    #[test]
    fn test_concat() {
        // 2, 1
        let e = List::one(1).cons(2);
        // 4, 3
        let e2 = List::one(3).cons(4);
        // 2, 1, 4, 3
        let e = e.concat(&e2);
        assert_eq!(e.head(), &2);
        assert_eq!(e.tail().tail().head(), &4);
    }

    #[test]
    fn test_update_at() {
        // 2, 1, 4, 3
        let e = List::one(3).cons(4).cons(1).cons(2);
        // 2, 5, 4, 3
        let e = e.update_at(1, 5);
        assert_eq!(e.tail().head(), &5);
    }

    #[test]
    fn test_suffixes() {
        // 2, 1, 4, 3
        let e = List::one(3).cons(4).cons(1).cons(2);
        let e = e.suffixes();
        assert_eq!(e.tail().head().tail().head(), &4);
    }

    #[test]
    fn test_rev() {
        // 2, 1, 4, 3
        let e = List::one(3).cons(4).cons(1).cons(2);
        let e = e.rev();
        assert_eq!(e.head(), &3);
        let e = e.tail();
        assert_eq!(e.head(), &4);
        let e = e.tail();
        assert_eq!(e.head(), &1);
        let e = e.tail();
        assert_eq!(e.head(), &2);

        assert!(List::<i32>::empty().rev().is_empty());
    }

    #[test]
    fn test_root() {
        let e = List::empty();
        match *e.root() {
            Nil => {},
            _ => assert!(false),
        };
        let e = e.cons(1);
        match *e.root() {
            Nil => assert!(false),
            _ => {},
        };
    }
}
