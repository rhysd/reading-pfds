// p.39 - p.41
//
// Lazy evaluation

use std::boxed::Box;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

enum Value<'a, T: 'a> {
    NotYet(Box<Fn() -> T + 'a>),
    Memo(T),
}

use self::Value::*;

pub struct Thunk<'a, T: 'a> {
    boxed: RefCell<Box<Value<'a, T>>>
}

impl<'a, T: 'a> Thunk<'a, T> {
    pub fn new<F>(f: F) -> Self where F: Fn() -> T + 'a {
        Thunk { boxed: RefCell::new(Box::new(NotYet(Box::new(f)))) }
    }

    pub fn force(&self) {
        let mut boxed = &mut *self.boxed.borrow_mut();
        let val = match **boxed {
            NotYet(ref invoke) => {
                Box::new(Memo(invoke()))
            },
            Memo(_) => return,
        };
        *boxed = val;
    }
}

impl<'a, T: 'a> Deref for Thunk<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.force();
        let boxed = unsafe {
            self.boxed.as_ptr().as_ref().unwrap()
        };
        match **boxed {
            Memo(ref v) => v,
            _ => unreachable!(),
        }
    }
}

impl<'a, T: 'a> DerefMut for Thunk<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.force();
        let boxed = unsafe {
            self.boxed.as_ptr().as_mut().unwrap()
        };
        match **boxed {
            Memo(ref mut v) => v,
            _ => unreachable!(),
        }
    }
}

#[macro_export]
macro_rules! lazily {
    ($e:expr) => {
        self::Thunk::new(move || { $e })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let i = lazily!("this expression will be evaluated lazily!".to_string());

        // The expression is evaluated (forced) at firstly accessed.
        let j = i.as_str();
        assert_eq!(j, "this expression will be evaluated lazily!");

        // It returns memo instead of evaluating the expression at second access.
        let k = i.as_str();
        assert_eq!(k, "this expression will be evaluated lazily!");

        let mut i = lazily!("this expression will be evaluated lazily!".to_string());
        let mut j = i.as_str();
        assert_eq!(j, "this expression will be evaluated lazily!");
        let mut k = i.as_str();
        assert_eq!(k, "this expression will be evaluated lazily!");
    }
}
