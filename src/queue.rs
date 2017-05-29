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

    pub fn enqueue(&self, x: T) -> Self {
        Queue::check(self.f.clone(), self.r.cons(x))
    }

    pub fn top(&self) -> &T {
        if self.f.is_empty() {
            panic!("Queue is empty")
        } else {
            self.f.head()
        }
    }

    pub fn dequeue(&self) -> Self {
        match self.f.root() {
            &Node::Nil => panic!("Queue is empty"),
            &Node::Cons(_, ref xs) => Queue::check(xs.clone(), self.r.clone()),
        }
    }
}
