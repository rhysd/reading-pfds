// p.30 - p.
//
// Binomial heap

use std::fmt::Debug;
use std::cmp::{Ord, Ordering};
use std::rc::Rc;
use list::{List, Node};

type TreeNodes<T> = List<TreeNode<T>>;

#[derive(Clone, Debug)]
struct TreeNode<T: Clone + Ord + Debug> {
    val: T,
    children: TreeNodes<T>,
}

impl<T> TreeNode<T>
where T: Clone + Ord + Debug {
    fn cons_child(&self, t: &TreeNode<T>) -> TreeNode<T> {
        TreeNode {
            val: self.val.clone(),
            children: self.children.cons(t.clone()),
        }
    }
}

// exercise 3.6: remove rank from each node of tree
#[derive(Clone, Debug)]
struct Tree<T: Clone + Ord + Debug> {
    rank: i32,
    root: TreeNode<T>,
}

impl<T> Tree<T>
where T: Clone + Ord + Debug {
    fn link(&self, other: &Self) -> Self {
        assert_eq!(self.rank, other.rank);

        let root = if self.root.val <= other.root.val {
            self.root.cons_child(&other.root)
        } else {
            other.root.cons_child(&self.root)
        };

        Tree {rank: self.rank + 1, root}
    }
}

type Trees<T> = List<Tree<T>>;

// Binomial heap is a sorted list of binomial trees whose ranks are not the same each other.
#[derive(Debug)]
pub struct BinHeap<T: Clone + Ord + Debug> {
    trees: Trees<T>,
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
        let t = Tree {
            rank: 0,
            root: TreeNode{val: v, children: List::empty()},
        };
        heap(BinHeap::insert_tree(t, &self.trees))
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

    fn find_min_root(trees: &Trees<T>) -> &Tree<T> {
        match *trees.0 {
            Node::Nil => panic!("No tree in heap!"),
            Node::Cons(ref t, ref ts) if ts.is_empty() => t,
            Node::Cons(ref t, ref ts) => {
                let t2 = BinHeap::find_min_root(ts);
                if t.root.val <= t2.root.val { t } else { t2 }
            },
        }
    }

    // exercise 3.5: Implement find_min without remove_min_root
    pub fn find_min(&self) -> &T {
        &BinHeap::find_min_root(&self.trees).root.val
    }

    fn remove_min_root(trees: &Trees<T>) -> (&Tree<T>, Trees<T>) {
        match *trees.0 {
            Node::Nil => panic!("No tree in heap!"),
            Node::Cons(ref t, ref ts) if ts.is_empty() => (t, List::empty()),
            Node::Cons(ref t, ref ts) => {
                let (t2, ts2) = BinHeap::remove_min_root(ts);
                if t.root.val <= t2.root.val {
                    (t, ts.clone())
                } else {
                    (t2, ts2.cons(t.clone()))
                }
            }
        }
    }

    fn nodes_to_trees(rank: i32, ts: &TreeNodes<T>) -> Trees<T> {
        match *ts.0 {
            Node::Nil => List::empty(),
            Node::Cons(ref t, ref ts) => {
                let ts = BinHeap::nodes_to_trees(rank, ts);
                List(Rc::new(Node::Cons(Tree{rank, root: t.clone()}, ts)))
            },
        }
    }

    pub fn delete_min(&self) -> Self {
        // Note: Rust's pattern cannot contain both by-ref and by-move binding at the same time.
        let (t, ts1) = BinHeap::remove_min_root(&self.trees);
        let &Tree{rank: ref rank, root: TreeNode{val: _, children: ref children}} = t;
        let ts2 = BinHeap::nodes_to_trees(rank - 1, children);
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
