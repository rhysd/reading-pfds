// p.27 - p.30
//
// Leftist heap

use std::fmt::Debug;
use std::rc::Rc;
use std::cmp::Ord;

// Right spine is a rank of heap

#[derive(Debug, Clone)]
enum LeftHeap<T: Ord + Clone + Debug> {
    Leaf,
    Knot(i32, T, Rc<LeftHeap<T>>, Rc<LeftHeap<T>>),
}

impl<T> LeftHeap<T>
where T: Ord + Clone + Debug {
    pub fn empty() -> Self {
        LeftHeap::Leaf
    }

    pub fn rank(&self) -> i32 {
        match *self {
            LeftHeap::Leaf => 0,
            LeftHeap::Knot(r, _, _, _) => r,
        }
    }

    pub fn is_empty(&self) -> bool {
        // T does not derive PartialEq so == is not available
        match *self {
            LeftHeap::Leaf => true,
            _ => false,
        }
    }

    fn make_knot(x: T, a: Rc<LeftHeap<T>>, b: Rc<LeftHeap<T>>) -> Self {
        if a.rank() >= b.rank() {
            LeftHeap::Knot(b.rank() + 1, x, a, b)
        } else {
            LeftHeap::Knot(a.rank() + 1, x, b, a)
        }
    }

    pub fn merge(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (&LeftHeap::Leaf, _) => rhs.clone(),
            (_, &LeftHeap::Leaf) => self.clone(),
            (&LeftHeap::Knot(_, ref x, ref a1, ref b1), &LeftHeap::Knot(_, ref y, ref a2, ref b2)) => {
                if x <= y {
                    LeftHeap::make_knot(x.clone(), a1.clone(), Rc::new(b1.merge(rhs)))
                } else {
                    LeftHeap::make_knot(y.clone(), a2.clone(), Rc::new(self.merge(b2)))
                }
            }
        }
    }

    // exersize 3.2: implement insert() without merge()
    pub fn insert(&self, v: T) -> Self {
        match *self {
            LeftHeap::Leaf => LeftHeap::Knot(1, v, Rc::new(LeftHeap::Leaf), Rc::new(LeftHeap::Leaf)),
            LeftHeap::Knot(_, ref x, ref a, ref b) => {
                if *x <= v {
                    LeftHeap::make_knot(x.clone(), a.clone(), Rc::new(b.insert(v)))
                } else {
                    LeftHeap::make_knot(v, Rc::new(LeftHeap::Leaf), Rc::new(self.clone()))
                }
            },
        }
    }

    pub fn find_min(&self) -> &T {
        match *self {
            LeftHeap::Leaf => panic!("heap is empty!"),
            LeftHeap::Knot(_, ref x, _, _) => x,
        }
    }

    pub fn delete_min(&self) -> Self {
        match *self {
            LeftHeap::Leaf => panic!("heap is empty!"),
            LeftHeap::Knot(_, _, ref a, ref b) => a.merge(b)
        }
    }

    // exersize 3.3
    pub fn from_slice(a: &[T]) -> Self {
        let mut heaps = a.iter()
            .map(|e| LeftHeap::Knot(1, e.clone(), Rc::new(LeftHeap::Leaf), Rc::new(LeftHeap::Leaf)))
            .collect::<Vec<_>>();

        while heaps.len() > 1 {
            heaps = heaps.chunks(2).map(|pair| {
                if pair.len() == 1 {
                    pair[0].clone()
                } else {
                    pair[0].merge(&pair[1])
                }
            }).collect::<Vec<_>>();
        }

        heaps.pop().expect("heap is empty")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let e = LeftHeap::<i32>::empty();
        assert!(e.is_empty());
        let e = e.merge(&e);
        assert!(e.is_empty());
    }

    #[test]
    fn test_merge() {
        let h1 = LeftHeap::empty().insert(4).insert(6).insert(10);
        let h2 = LeftHeap::empty().insert(1).insert(7).insert(9);
        let mut h = h1.merge(&h2);
        for i in &[1, 4, 6, 7, 9, 10] {
            assert_eq!(h.find_min(), i);
            h = h.delete_min();
        }
        h = h1.merge(&h2);
        for i in &[1, 4, 6, 7, 9, 10] {
            assert_eq!(h.find_min(), i);
            h = h.delete_min();
        }
        h = h.insert(0);
        assert_eq!(h.find_min(), &0);
    }

    #[test]
    fn test_insert() {
        let mut h = LeftHeap::empty().insert(4).insert(6).insert(10);
        h = h.insert(1);
        h = h.insert(7);
        h = h.insert(12);
        for i in &[1, 4, 6, 7, 10, 12] {
            assert_eq!(h.find_min(), i);
            h = h.delete_min();
        }
        assert!(h.is_empty());
    }

    #[test]
    fn test_from_array() {
        let mut h = LeftHeap::from_slice(&[4, 10, 6, 1, 9]);
        for i in &[1, 4, 6, 9, 10] {
            assert_eq!(h.find_min(), i);
            h = h.delete_min();
        }
        assert!(h.is_empty());
    }
}
