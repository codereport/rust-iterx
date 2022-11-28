use std::iter::Scan;

#[cfg(test)]
use std::ops::Add;

pub fn option_lift<T: Copy>(f: &dyn Fn(&mut T, T) -> T) -> impl Fn(&mut T, T) -> Option<T> + '_ {
    |acc, x| {
        *acc = f(&mut *acc, x);
        Some(*acc)
    }
}

#[cfg(test)]
fn w<T: Copy, O>(binop: &dyn Fn(T, T) -> O) -> impl Fn(T) -> O + '_ {
    |x| binop(x, x)
}

// pub trait Iterscans : Iterator {
//     ✅ fn scan_while(self, f: F) {}
//     ✅ fn scan_(self, f: F) {}
//     fn prescan_while(self, init: T, f: F) {}
//     fn prescan(self, init: T, f: F) {}
// }

pub trait Iterscans: Iterator {
    // Name scan_ so as to not collide with std::iter::Iterator::sc an
    // std::iter::Iterator::scan should really be scan_while
    fn scan_<F>(self, f: F) -> Prescan<Self, Self::Item, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Self::Item,
    {
        Prescan::new(self, f)
    }

    // A renaming of https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html#1420
    fn scan_while<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
    where
        Self: Sized,
        F: FnMut(&mut St, Self::Item) -> Option<B>,
    {
        self.scan(initial_state, f)
    }
}

impl<T: ?Sized> Iterscans for T where T: Iterator {}

#[derive(Clone)]
pub struct Prescan<I, T, F> {
    iter: I,
    f: F,
    state: Option<T>,
    is_first: bool,
}

impl<I, T, F> Prescan<I, T, F>
where
    I: Iterator<Item = T>,
{
    fn new(iter: I, f: F) -> Self {
        Self {
            iter,
            f,
            state: None,
            is_first: true,
        }
    }
}

impl<I, T, F> Iterator for Prescan<I, T, F>
where
    I: Iterator<Item = T>,
    F: FnMut(&T, &T) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.state = Some(self.iter.next())?;
            self.is_first = false;
        }

        let state = self.state.take()?;

        if let Some(x) = self.iter.next() {
            self.state = Some((self.f)(&state, &x));
        }

        Some(state)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use itertools::assert_equal;

    #[test]
    fn test_scan_not_what_i_want() {
        let input = vec![1, 1, 1];
        let expected = vec![1, 2, 3];
        assert_equal(
            input
                .into_iter()
                .scan_while(0, &option_lift(&|a, b| *a + b)),
            expected,
        );
    }

    #[test]
    fn test_w_combinator() {
        assert_eq!(w(&Add::add)(1), 2);
    }

    #[test]
    fn test_scanl1() {
        assert_equal(vec![1, 1, 1].into_iter().scan_(|x, y| x + y), vec![1, 2, 3]);
        assert_equal((1..=5).scan_(|x, y| x + y), vec![1, 3, 6, 10, 15]);
    }
}
