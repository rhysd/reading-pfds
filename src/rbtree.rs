// p.33 - p.
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
    // fn black(left: Link<T>, right: Link<T>, val: T) -> Self
    // fn red(left: Link<T>, right: Link<T>, val: T) -> Self
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
    fn balance(color: Color, left: Link<T>, right: Link<T>, val: T) -> Node<T> {
        if color == Color::Red {
            return Node::Knot{color, left, right, val};
        }

        if let Node::Knot{color: Color::Red, left: ref l1, right: ref r1, val: ref v1} = *left {
            if let Node::Knot{color: Color::Red, left: ref l2, right: ref r2, val: ref v2} = **l1 {
                return Node::Knot{
                    color: Color::Red,
                    left: Rc::new(Node::Knot{color: Color::Black, left: l2.clone(), right: r2.clone(), val: v2.clone()}),
                    right: Rc::new(Node::Knot{color: Color::Black, left: r1.clone(), right, val}),
                    val: v1.clone(),
                };
            }
            if let Node::Knot{color: Color::Red, left: ref l2, right: ref r2, val: ref v2} = **r1 {
                return Node::Knot{
                    color: Color::Red,
                    left: Rc::new(Node::Knot{color: Color::Black, left: l1.clone(), right: l2.clone(), val: v1.clone()}),
                    right: Rc::new(Node::Knot{color: Color::Black, left: r2.clone(), right, val}),
                    val: v2.clone(),
                };
            }
        } if let Node::Knot{color: Color::Red, left: ref l1, right: ref r1, val: ref v1} = *right {
            if let Node::Knot{color: Color::Red, left: ref l2, right: ref r2, val: ref v2} = **l1 {
                return Node::Knot{
                    color: Color::Red,
                    left: Rc::new(Node::Knot{color: Color::Black, left, right: l2.clone(), val}),
                    right: Rc::new(Node::Knot{color: Color::Black, left: r2.clone(), right: r1.clone(), val: v1.clone()}),
                    val: v2.clone(),
                };
            }
            if let Node::Knot{color: Color::Red, left: ref l2, right: ref r2, val: ref v2} = **r1 {
                return Node::Knot{
                    color: Color::Red,
                    left: Rc::new(Node::Knot{color: Color::Black, left, right: l1.clone(), val}),
                    right: Rc::new(Node::Knot{color: Color::Black, left: l2.clone(), right: r2.clone(), val: v2.clone()}),
                    val: v1.clone(),
                };
            }
        }

        return Node::Knot{color, left, right, val};
    }

    fn ins(link: &Link<T>, x: T) -> Link<T> {
        match **link {
            Node::Leaf => {
                let e = Rc::new(Node::Leaf);
                Rc::new(Node::Knot{color: Color::Red, left: e.clone(), right: e, val: x})
            },
            Node::Knot{ref color, ref left, ref right, ref val} => {
                let color = color.clone();
                let val = val.clone();
                if x < val {
                    let left = RBTree::ins(left, x);
                    Rc::new(RBTree::balance(color, left, right.clone(), val))
                } else if val < x {
                    let right = RBTree::ins(right, x);
                    Rc::new(RBTree::balance(color, left.clone(), right, val))
                } else {
                    link.clone()
                }
            },
        }
    }

    pub fn insert(&self, v: T) -> Self {
        RBTree{root: RBTree::ins(&self.root, v)}
    }
}
