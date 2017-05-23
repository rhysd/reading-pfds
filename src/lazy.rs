// p.39 - p.41
//
// Lazy evaluation for immutable data structures

use std::fmt;
use std::fmt::Debug;
use std::boxed::Box;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

enum Thunk<'a, T: 'a + Debug> {
    NotYet(Box<Fn() -> T + 'a>),
    Memo(T),
}

// Note: Cannot derive std::format::Debug because of Fn.
impl<'a, T> fmt::Debug for Thunk<'a, T>
where T: 'a + Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &NotYet(_) => write!(f, "Thunk {{ (not yet...) }}"),
            &Memo(ref v) => write!(f, "Thunk {{ {:?} }}", v),
        }
    }
}

use self::Thunk::*;

// Note:
// Need to use Box<T> instead of Rc<T> here for implementing Clone trait. Clone is needed for copying
// elements of immutable data structures.
#[derive(Clone, Debug)]
pub struct Delayed<'a, T: 'a + Debug> {
    thunk: RefCell<Rc<Thunk<'a, T>>>
}

impl<'a, T: 'a + Debug> Delayed<'a, T> {
    pub fn new<F>(f: F) -> Self where F: Fn() -> T + 'a {
        Delayed { thunk: RefCell::new(Rc::new(NotYet(Box::new(f)))) }
    }

    pub fn constant(v: T) -> Self {
        Delayed { thunk: RefCell::new(Rc::new(Memo(v))) }
    }

    pub fn force(&self) {
        let thunk = &mut *self.thunk.borrow_mut();
        let val = match **thunk {
            NotYet(ref invoke) => {
                Rc::new(Memo(invoke()))
            },
            Memo(_) => return,
        };
        *thunk = val;
    }

    // Note:
    // I gave up using Deref for Delayed. If implementing Deref value,  accessing to reference of
    // Delayed value immediately evaluates its delayed expression. In general, reference of type which
    // implements Deref cannot be obtained because getting reference with `&` coerces into the value
    // into target type.
    pub fn eval(&self) -> &T {
        self.force();
        let thunk = unsafe {
            self.thunk.as_ptr().as_ref().unwrap()
        };
        match **thunk {
            Memo(ref v) => v,
            _ => unreachable!(),
        }
    }
}

// Note:
// I needed Rc<RefCell<T>> for implementing eval_mut().
// We could have implemented eval_mut() when using Box instead of Rc previously. This had been because
// Box propagates mutability of content (`mut Box<T>` means `mut Box<mut T>`) but Rc does not. In other
// words, Rc does not implement eval_mut().
//
// Since I don't use Delayed with mutable expression because we implement immutabile data structures,
// I skipped to implement DrefMut simply.

#[macro_export]
macro_rules! lazily {
    ($($b:tt)+) => {
        self::Delayed::new(move || { $($b)+ })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let i = lazily!{"this expression will be evaluated lazily!".to_string()};

        // The expression is evaluated (forced) at firstly accessed.
        let j = i.eval().as_str();
        assert_eq!(j, "this expression will be evaluated lazily!");

        // It returns memo instead of evaluating the expression at second access.
        let k = i.eval().as_str();
        assert_eq!(k, "this expression will be evaluated lazily!");

        // A bit complicated example: Calculate 1000th primer number
        let prime1000 = lazily! {
            let mut known = vec![2];
            let mut i = 3;
            while known.len() <= 1000 {
                if known.iter().all(|p| i % p != 0) {
                    known.push(i);
                }
                i += 1;
            }
            *known.last().unwrap()
        };
        assert_eq!(*prime1000.eval(), 7927);
        assert_eq!(*prime1000.eval(), 7927);

        // Inner infinite loop will never be evaluated until the delayed computation is invoked.
        let _ = lazily! {
            loop {}
        };
    }
}
