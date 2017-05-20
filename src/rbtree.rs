// p.33 - p.38
//
// Red-Black tree

use std::fmt::Debug;
use std::rc::Rc;
use std::cmp::PartialOrd;

#[derive(Clone, Debug, PartialEq)]
enum Color {Red, Black}

#[derive(Clone, Debug, PartialEq)]
enum Node<T: Clone + PartialOrd + Debug> {
    Leaf, // Note: Leaf is always black
    Knot{
        color: Color,
        left: Link<T>,
        right: Link<T>,
        val: T,
    },
}

impl<T> Node<T>
where T: Clone + PartialOrd + Debug {
    fn member(&self, v: &T) -> bool {
        match *self {
            Node::Leaf => false,
            Node::Knot{color: _, ref left, ref right, ref val} => {
                if val < v {
                    right.member(v)
                } else if v < val {
                    left.member(v)
                } else {
                    // val == v
                    true
                }
            }
        }
    }
}

fn black<T: Clone + PartialOrd + Debug>(left: Link<T>, right: Link<T>, val: T) -> Link<T> {
    Rc::new(Node::Knot{color: Color::Black, left, right, val})
}
fn red<T: Clone + PartialOrd + Debug>(left: Link<T>, right: Link<T>, val: T) -> Link<T> {
    Rc::new(Node::Knot{color: Color::Red, left, right, val})
}

type Link<T> = Rc<Node<T>>;

#[derive(Clone, Debug)]
pub struct RBTree<T: Clone + PartialOrd + Debug> {
    root: Link<T>,
}

impl<T> RBTree<T>
where T: Clone + PartialOrd + Debug {
    pub fn empty() -> Self {
        RBTree{root: Rc::new(Node::Leaf)}
    }

    pub fn is_empty(&self) -> bool {
        *self.root == Node::Leaf
    }

    pub fn member(&self, v: &T) -> bool {
        self.root.member(v)
    }

    // exercise 3.10: Eliminate redundant comparison
    fn balance(color: Color, left: Link<T>, right: Link<T>, val: T) -> Link<T> {
        if color == Color::Red {
            return Rc::new(Node::Knot{color, left, right, val});
        }

        if let Node::Knot{color: Color::Red, left: ref l1, right: ref r1, val: ref v1} = *left {
            if let Node::Knot{color: Color::Red, left: ref l2, right: ref r2, val: ref v2} = **l1 {
                return red(
                    black(l2.clone(), r2.clone(), v2.clone()),
                    black(r1.clone(), right, val),
                    v1.clone(),
                );
            }
            if let Node::Knot{color: Color::Red, left: ref l2, right: ref r2, val: ref v2} = **r1 {
                return red(
                    black(l1.clone(), l2.clone(), v1.clone()),
                    black(r2.clone(), right, val),
                    v2.clone(),
                );
            }
        } if let Node::Knot{color: Color::Red, left: ref l1, right: ref r1, val: ref v1} = *right {
            if let Node::Knot{color: Color::Red, left: ref l2, right: ref r2, val: ref v2} = **l1 {
                return red(
                    black(left, l2.clone(), val),
                    black(r2.clone(), r1.clone(), v1.clone()),
                    v2.clone(),
                );
            }
            if let Node::Knot{color: Color::Red, left: ref l2, right: ref r2, val: ref v2} = **r1 {
                return red(
                    black(left, l1.clone(), val),
                    black(l2.clone(), r2.clone(), v2.clone()),
                    v1.clone(),
                );
            }
        }

        return Rc::new(Node::Knot{color, left, right, val});
    }

    fn ins(link: &Link<T>, x: T) -> Link<T> {
        match **link {
            Node::Leaf => {
                let e = Rc::new(Node::Leaf);
                red(e.clone(), e, x)
            },
            Node::Knot{ref color, ref left, ref right, ref val} => {
                let color = color.clone();
                let val = val.clone();
                if x < val {
                    let left = RBTree::ins(left, x);
                    RBTree::balance(color, left, right.clone(), val)
                } else if val < x {
                    let right = RBTree::ins(right, x);
                    RBTree::balance(color, left.clone(), right, val)
                } else {
                    link.clone()
                }
            },
        }
    }

    pub fn insert(&self, v: T) -> Self {
        match *RBTree::ins(&self.root, v) {
            Node::Knot{color: _, left: ref l, right: ref r, val: ref v} => {
                RBTree{root: black(l.clone(), r.clone(), v.clone())}
            },
            Node::Leaf => unreachable!(),
        }
    }

    fn link_from_sorted(idx: usize, slice: &[T]) -> (Link<T>, usize) {
        match slice.get(idx) {
            None => (Rc::new(Node::Leaf), idx),
            Some(val) => {
                let (left, idx) = RBTree::link_from_sorted(idx + 1, slice);
                let (right, idx) = RBTree::link_from_sorted(idx + 1, slice);
                let color = match &*left {
                    &Node::Leaf => Color::Red,
                    &Node::Knot{color: Color::Red, ..} => Color::Black,
                    &Node::Knot{color: Color::Black, ..} => Color::Red,
                };
                (Rc::new(Node::Knot{color, left, right, val: val.clone()}), idx)
            }
        }
    }

    // exercise 3.9: Create RBTree from ordered unique list
    pub fn from_sorted(slice: &[T]) -> Self {
        let (root, _) = RBTree::link_from_sorted(0, slice);
        RBTree{root}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = RBTree::empty();
        assert!(t.is_empty());
        let t = t.insert(10).insert(3).insert(7).insert(1).insert(9);
        assert!(!t.is_empty());
        match *t.root {
            Node::Leaf => assert!(false),
            Node::Knot{color: _, left: _, right: _, val} => {
                assert_eq!(val, 7);
            },
        }
    }
}
