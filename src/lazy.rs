// p.39 - p.41
//
// Lazy evaluation

use std::boxed::Box;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

enum Thunk<'a, T: 'a> {
    NotYet(Box<Fn() -> T + 'a>),
    Memo(T),
}

use self::Thunk::*;

pub struct Delayed<'a, T: 'a> {
    thunk: RefCell<Box<Thunk<'a, T>>>
}

impl<'a, T: 'a> Delayed<'a, T> {
    pub fn new<F>(f: F) -> Self where F: Fn() -> T + 'a {
        Delayed { thunk: RefCell::new(Box::new(NotYet(Box::new(f)))) }
    }

    pub fn force(&self) {
        let mut thunk = &mut *self.thunk.borrow_mut();
        let val = match **thunk {
            NotYet(ref invoke) => {
                Box::new(Memo(invoke()))
            },
            Memo(_) => return,
        };
        *thunk = val;
    }
}

impl<'a, T: 'a> Deref for Delayed<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
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

impl<'a, T: 'a> DerefMut for Delayed<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.force();
        let thunk = unsafe {
            self.thunk.as_ptr().as_mut().unwrap()
        };
        match **thunk {
            Memo(ref mut v) => v,
            _ => unreachable!(),
        }
    }
}

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
        let j = i.as_str();
        assert_eq!(j, "this expression will be evaluated lazily!");

        // It returns memo instead of evaluating the expression at second access.
        let k = i.as_str();
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
        assert_eq!(*prime1000, 7927);
        assert_eq!(*prime1000, 7927);

        // Inner infinite loop will never be evaluated until the delayed computation is invoked.
        let _ = lazily! {
            loop {}
        };

        // Check mutable value is also OK
        let mut i = lazily!{"this expression will be evaluated lazily!".to_string()};
        let mut j = i.as_str();
        assert_eq!(j, "this expression will be evaluated lazily!");
        let mut k = i.as_str();
        assert_eq!(k, "this expression will be evaluated lazily!");
    }
}
