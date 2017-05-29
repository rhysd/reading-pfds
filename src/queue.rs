// p.45 - p.49
//
// Queue.
// Amortized O(1) operations (enqueue/dequeue)

use std::fmt::Debug;
use list::{List, Node};

// Note:
// Invariants: When f is empty, r must be empty also.
#[derive(Clone, Debug)]
pub struct Queue<T: Clone + Debug> {
    f: List<T>,
    r: List<T>,
}

impl<T> Queue<T>
where T: Clone + Debug {
    pub fn empty() -> Self {
        Queue{f: List::empty(), r: List::empty()}
    }

    pub fn is_empty(&self) -> bool {
        self.f.is_empty()
    }

    fn check(f: List<T>, r: List<T>) -> Self {
        if !f.is_empty() {
            Queue{f, r}
        } else {
            Queue {
                f: r.rev(),
                r: List::empty(),
            }
        }
    }

    pub fn enq(&self, x: T) -> Self {
        Queue::check(self.f.clone(), self.r.cons(x))
    }

    pub fn top(&self) -> &T {
        if self.f.is_empty() {
            panic!("Queue is empty")
        } else {
            self.f.head()
        }
    }

    pub fn deq(&self) -> Self {
        match self.f.root() {
            &Node::Nil => panic!("Queue is empty"),
            &Node::Cons(_, ref xs) => Queue::check(xs.clone(), self.r.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let q = Queue::empty();
        assert!(q.is_empty());
        let q = q.enq(1);
        assert!(!q.is_empty());
    }

    #[test]
    fn test_top() {
        let q = Queue::empty().enq(1);
        assert_eq!(q.top(), &1);
    }

    #[test]
    fn test_invariants() {
        let q = Queue::empty().enq(1).enq(2).enq(3);
        let q = q.deq();
        assert_eq!(q.top(), &2);
        let q = q.enq(4).deq().deq();
        assert_eq!(q.top(), &4);
        let q = q.deq();
        assert!(q.is_empty());
    }
}
