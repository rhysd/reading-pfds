// p.49
//
// Exercise 5.10: Amortized O(1) deque

use std::fmt::Debug;
use list::{List, Node};

// Invariant: When it contains two or more elements, both f and r contains at least one element.
#[derive(Clone, Debug)]
pub struct Deque<T: Clone + Debug> {
    f: List<T>,
    r: List<T>,
}

impl<T> Deque<T>
where T: Clone + Debug {
    pub fn empty() -> Self {
        let l = List::empty();
        Deque{f: l.clone(), r: l}
    }

    pub fn is_empty(&self) -> bool {
        self.f.is_empty() && self.r.is_empty()
    }

    fn make(f: List<T>, r: List<T>) -> Self {
        // Meet Invariant: Both f and r must have one or more elements if the
        // Deque has two or more elements.
        match (f.is_empty(), r.is_empty()) {
            (true, true) => Deque{f, r},
            (false, false) => Deque{f, r},
            (true, false) => Deque{f: r.tail().rev(), r: List::empty().cons(r.head().clone())},
            (false, true) => Deque{f: List::empty().cons(f.head().clone()), r: f.tail().rev()},
        }
    }

    pub fn enq_front(&self, x: T) -> Self {
        Deque::make(self.f.cons(x), self.r.clone())
    }

    pub fn enq_back(&self, x: T) -> Self {
        Deque::make(self.f.clone(), self.r.cons(x))
    }

    pub fn deq_front(&self) -> Self {
        match self.f.root() {
            &Node::Nil => {
                if self.r.is_empty() {
                    panic!("Deque is empty!")
                }
                // Only one element is in deque due to invariant
                Deque::empty()
            },
            &Node::Cons(_, ref xs) => {
                Deque::make(xs.clone(), self.r.clone())
            },
        }
    }

    pub fn deq_back(&self) -> Self {
        match self.r.root() {
            &Node::Nil => {
                if self.f.is_empty() {
                    panic!("Deque is empty!")
                }
                // Only one element is in deque due to invariant
                Deque::empty()
            },
            &Node::Cons(_, ref xs) => {
                Deque::make(self.f.clone(), xs.clone())
            },
        }
    }

    pub fn front(&self) -> &T {
        match self.f.root() {
            &Node::Nil => {
                if self.r.is_empty() {
                    panic!("Deque is empty!")
                }
                // Only one element is in deque due to invariant
                self.r.head()
            },
            &Node::Cons(ref x, _) => x,
        }
    }

    pub fn back(&self) -> &T {
        match self.r.root() {
            &Node::Nil => {
                if self.f.is_empty() {
                    panic!("Deque is empty!")
                }
                // Only one element is in deque due to invariant
                self.f.head()
            },
            &Node::Cons(ref x, _) => x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let d = Deque::empty();
        assert!(d.is_empty());
        let d = d.enq_front(1);
        assert!(!d.is_empty());
    }

    #[test]
    fn test_front_to_back() {
        let d = Deque::empty().enq_front(1);
        assert_eq!(d.front(), &1);
        assert_eq!(d.back(), &1);

        let d = d.enq_front(2).enq_front(3);
        assert_eq!(d.front(), &3);
        assert_eq!(d.back(), &1);

        let d = d.deq_back();
        assert_eq!(d.front(), &3);
        assert_eq!(d.back(), &2);

        let d = d.deq_back().deq_back();
        assert!(d.is_empty());
    }

    #[test]
    fn test_back_to_front() {
        let d = Deque::empty().enq_back(1);
        assert_eq!(d.front(), &1);
        assert_eq!(d.back(), &1);

        let d = d.enq_back(2).enq_back(3);
        assert_eq!(d.front(), &1);
        assert_eq!(d.back(), &3);

        let d = d.deq_front();
        assert_eq!(d.front(), &2);
        assert_eq!(d.back(), &3);

        let d = d.deq_front().deq_front();
        assert!(d.is_empty());
    }

    #[test]
    fn test_enq_both_sides() {
        let d = Deque::empty().enq_front(2).enq_back(3).enq_front(1).enq_back(4);
        assert_eq!(d.front(), &1);
        assert_eq!(d.back(), &4);
        let d = d.deq_back().deq_front().deq_back().deq_front();
        assert!(d.is_empty());
    }
} // mod tests
