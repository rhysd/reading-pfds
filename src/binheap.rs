// p.30 - p.
//
// Binomial heap

use std::fmt::Debug;
use std::cmp::{Ord, Ordering};
use list::{List, Node};

type Trees<T> = List<Tree<T>>;

#[derive(Clone, Debug)]
struct Tree<T: Clone + Ord + Debug> {
    rank: i32,
    val: T,
    children: Trees<T>,
}

// Binomial heap is a sorted list of binomial trees whose ranks are not the same each other.
#[derive(Debug)]
pub struct BinHeap<T: Clone + Ord + Debug> {
    trees: Trees<T>,
}

impl<T> Tree<T>
where T: Clone + Ord + Debug {
    fn link(&self, other: &Self) -> Self {
        assert_eq!(self.rank, other.rank);
        if self.val <= other.val {
            Tree {
                rank: self.rank + 1,
                val: self.val.clone(),
                children: self.children.cons(other.clone()),
            }
        } else {
            Tree {
                rank: self.rank + 1,
                val: other.val.clone(),
                children: other.children.cons(self.clone()),
            }
        }
    }
}

fn heap<T: Clone + Ord + Debug>(trees: Trees<T>) -> BinHeap<T> {
    BinHeap {trees}
}

impl<T> BinHeap<T>
where T: Clone + Ord + Debug {
    pub fn empty() -> Self {
        heap(List::empty())
    }

    fn insert_tree(t: Tree<T>, ts: &Trees<T>) -> Trees<T> {
        match *ts.0 {
            Node::Nil => ts.cons(t),
            Node::Cons(ref t2, ref ts2) => {
                if t.rank < t2.rank {
                    ts.cons(t)
                } else {
                    BinHeap::insert_tree(t.link(t2), ts2)
                }
            }
        }
    }

    pub fn insert(&self, v: T) -> Self {
        heap(BinHeap::insert_tree(Tree{rank: 0, val: v, children: List::empty()}, &self.trees))
    }

    fn merge_trees(ts1: &Trees<T>, ts2: &Trees<T>) -> Trees<T> {
        match (&*ts1.0, &*ts2.0) {
            (&Node::Nil, _) => ts2.clone(),
            (_, &Node::Nil) => ts1.clone(),
            (&Node::Cons(ref x1, ref xs1), &Node::Cons(ref x2, ref xs2)) => {
                match x1.rank.cmp(&x2.rank) {
                    Ordering::Less => BinHeap::merge_trees(xs1, ts2).cons(x1.clone()),
                    Ordering::Greater => BinHeap::merge_trees(ts1, xs2).cons(x2.clone()),
                    Ordering::Equal => BinHeap::insert_tree(x1.link(x2), &BinHeap::merge_trees(xs1, xs2)),
                }
            },
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        heap(BinHeap::merge_trees(&self.trees, &other.trees))
    }

    fn find_min_tree(trees: &Trees<T>) -> &Tree<T> {
        match *trees.0 {
            Node::Nil => panic!("No tree in heap!"),
            Node::Cons(ref t, ref ts) if ts.is_empty() => t,
            Node::Cons(ref t, ref ts) => {
                let t2 = BinHeap::find_min_tree(ts);
                if t.val <= t2.val { t } else { t2 }
            },
        }
    }

    // exercise 3.5: Implement find_min without remove_min_tree
    pub fn find_min(&self) -> &T {
        &BinHeap::find_min_tree(&self.trees).val
    }

    fn remove_min_tree(trees: &Trees<T>) -> (&Tree<T>, Trees<T>) {
        match *trees.0 {
            Node::Nil => panic!("No tree in heap!"),
            Node::Cons(ref t, ref ts) if ts.is_empty() => (t, List::empty()),
            Node::Cons(ref t, ref ts) => {
                let (t2, ts2) = BinHeap::remove_min_tree(ts);
                if t.val <= t2.val {
                    (t, ts.clone())
                } else {
                    (t2, ts2.cons(t.clone()))
                }
            }
        }
    }

    pub fn delete_min(&self) -> Self {
        // Note: Rust's pattern cannot contain both by-ref and by-move binding at the same time.
        let (t, ts2) = BinHeap::remove_min_tree(&self.trees);
        let &Tree{rank: _, val: _, children: ref ts1} = t;
        heap(BinHeap::merge_trees(&ts1.rev(), &ts2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_empty() {
        let h = BinHeap::<i32>::empty();
        assert!(h.trees.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut h = BinHeap::empty().insert(3).insert(1).insert(7).insert(10);
        for i in &[1, 3, 7, 10] {
            assert_eq!(h.find_min(), i);
            h = h.delete_min();
        }
    }

    #[test]
    fn test_merge() {
        let h1 = BinHeap::empty().insert(3).insert(1).insert(7).insert(10);
        let h2 = BinHeap::empty().insert(2).insert(4).insert(11).insert(0);
        let mut h = h1.merge(&h2);
        for i in &[0, 1, 2, 3, 4, 7, 10, 11] {
            assert_eq!(h.find_min(), i);
            h = h.delete_min();
        }
        assert!(h.trees.is_empty());
        h = h2.merge(&h1);
        for i in &[0, 1, 2, 3, 4, 7, 10, 11] {
            assert_eq!(h.find_min(), i);
            h = h.delete_min();
        }
        assert!(h.trees.is_empty());
    }
}
