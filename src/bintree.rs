// p.21 - p.25

use std::fmt::Debug;
use std::rc::Rc;
use std::cmp::PartialOrd;

// left <= right
#[derive(Clone, PartialEq, Debug)]
enum BinTree<T: Clone + PartialOrd + Debug> {
    Leaf,
    Knot(T, Rc<BinTree<T>>, Rc<BinTree<T>>),
}

impl<T> BinTree<T>
where T: Clone + PartialOrd + Debug {
    pub fn from_array(arr: &[T]) -> Self {
        if arr.len() == 0 {
            BinTree::Leaf
        } else {
            BinTree::from_array(&arr[1..]).insert(arr[0].clone())
        }
    }

    pub fn member_impl(&self, v: &T, memo: Option<&T>) -> bool {
        match *self {
            BinTree::Leaf => {
                match memo {
                    // We already know v >= x. !(x < v) means x == v
                    Some(x) => !(x < v),
                    None => false,
                }
            },
            BinTree::Knot(ref x, ref l, ref r) => {
                // Compare element at once. It makes `member` d + 1 (not 2d) where d is depth.
                if v < x {
                    l.member_impl(v, memo)
                } else {
                    r.member_impl(v, Some(x))
                }
            }
        }
    }

    pub fn member(&self, v: &T) -> bool {
        self.member_impl(v, None)
    }

    pub fn insert_impl(&self, v: T, memo: Option<&T>) -> Option<Self> {
        match *self {
            BinTree::Leaf => {
                // exercise 2.4: Check memo to know there is an element which is equivalent to `v`.
                if let Some(x) = memo {
                    if !(x < &v) {
                        // exercise 2.3: If the tree already has the value, we don't need to clone tree at all.
                        return None;
                    }
                }
                Some(BinTree::Knot(v, Rc::new(BinTree::Leaf), Rc::new(BinTree::Leaf)))
            },
            BinTree::Knot(ref x, ref l, ref r) => {
                // exercise 2.4: Makes number of comparing element d + 1 (not 2d) where d is depth.
                if &v < x {
                    // Right of the node can be shared.
                    l.insert_impl(v, memo).map(|l| BinTree::Knot(x.clone(), Rc::new(l), r.clone()))
                } else {
                    // Left of the node can be shared.
                    r.insert_impl(v, Some(x)).map(|r| BinTree::Knot(x.clone(), l.clone(), Rc::new(r)))
                }
            },
        }
    }

    pub fn insert(self, v: T) -> Self {
        // exercise 2.3: If the tree already has the value, we don't need to clone tree at all.
        match (&self).insert_impl(v, None) {
            Some(t) => t,
            None => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_member() {
        let t = BinTree::from_array(&[5, 2, 4, 1, 6, 7]);
        assert!(t.member(&4));
        assert!(t.member(&1));
        assert!(t.member(&7));
        assert!(!t.member(&9));
    }

    #[test]
    fn test_insert() {
        let t = BinTree::from_array(&[5, 2, 4, 1, 6, 7]);
        let t = t.insert(10);
        assert!(t.member(&10));
        let t = t.insert(2);
        assert!(t.member(&2));
    }
}
