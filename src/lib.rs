use std::iter::{Map, Scan, Zip};

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

// TODO
// 1. Figure out &Add::add
// 2. Use in rust-tx

// pub trait Iterx : Iterator {
//     ✅ fn scan_while(self, f: F) {}
//     ✅ fn scan_(self, f: F) {}
//     fn prescan_while(self, init: T, f: F) {}
//     ✅ fn prescan(self, init: T, f: F) {}
// }

pub trait Iterx: Iterator {
    /// * Name scan_ so as to not collide with [`scan`][std::iter::Iterator::scan]
    /// * [`scan`][std::iter::Iterator::scan] has been renamed in this library to
    fn scan_<F>(self, f: F) -> Scan_<Self, Self::Item, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Self::Item,
    {
        Scan_::new(self, f)
    }

    /// * A renaming of [`scan`][std::iter::Iterator::scan]
    fn scan_while<St, B, F>(self, initial_state: St, f: F) -> Scan<Self, St, F>
    where
        Self: Sized,
        F: FnMut(&mut St, Self::Item) -> Option<B>,
    {
        self.scan(initial_state, f)
    }

    /// * The analog to `scan_` that takes an initial value.
    fn prescan<St, F>(self, initial_state: St, f: F) -> Prescan<Self, St, F>
    where
        Self: Sized,
        F: FnMut(&St, &Self::Item) -> St,
    {
        Prescan::new(self, initial_state, f)
    }

    /// * Fusion of [`map`][std::iter::Iterator::map] and [`zip`][std::iter::Iterator::zip] for convenience
    fn zip_map<U, T, F>(
        self,
        other: U,
        f: F,
    ) -> Map<
        Zip<Self, U::IntoIter>,
        Box<dyn Fn((<Self as Iterator>::Item, <U as IntoIterator>::Item)) -> T>,
    >
    where
        Self: Sized,
        U: IntoIterator,
        F: Fn(<Self as Iterator>::Item, <U as IntoIterator>::Item) -> T + 'static,
    {
        self.zip(other.into_iter())
            .map(Box::new(move |(x, y)| f(x, y)))
    }
}

impl<T: ?Sized> Iterx for T where T: Iterator {}

#[derive(Clone)]
pub struct Prescan<I, St, F> {
    iter: I,
    f: F,
    state: Option<St>,
}

impl<I, St, F> Prescan<I, St, F> {
    fn new(iter: I, initial_state: St, f: F) -> Self {
        Self {
            iter,
            f,
            state: Some(initial_state),
        }
    }
}

impl<I, St, F> Iterator for Prescan<I, St, F>
where
    I: Iterator,
    F: FnMut(&St, &I::Item) -> St,
{
    type Item = St;

    fn next(&mut self) -> Option<Self::Item> {
        let state = self.state.take()?;

        if let Some(x) = self.iter.next() {
            self.state = Some((self.f)(&state, &x));
        }

        Some(state)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lb, ub) = self.iter.size_hint();
        (lb + 1, ub.map(|ub| ub + 1))
    }
}

#[derive(Clone)]
pub struct Scan_<I, T, F> {
    iter: I,
    f: F,
    state: Option<T>,
    is_first: bool,
}

impl<I, T, F> Scan_<I, T, F>
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

impl<I, T, F> Iterator for Scan_<I, T, F>
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
        assert_equal(
            vec![1, 1, 1]
                .into_iter()
                .scan_while(0, &option_lift(&|a, b| *a + b)),
            1..=3,
        );
    }

    #[test]
    fn test_w_combinator() {
        assert_eq!(w(&Add::add)(1), 2);
    }

    #[test]
    fn test_scan_() {
        assert_equal(vec![1].into_iter().scan_(|x, y| x + y), 1..2);
        assert_equal(vec![1, 1, 1].into_iter().scan_(|x, y| x + y), 1..=3);
        assert_equal((1..=5).scan_(|x, y| x + y), vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_prescan() {
        assert_equal(vec![1].into_iter().prescan(0, |x, y| x + y), 0..=1);
        assert_equal(vec![1, 1, 1].into_iter().prescan(0, |x, y| x + y), 0..=3);
        assert_equal((1..=5).prescan(0, |x, y| x + y), vec![0, 1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_zip_map() {
        assert_equal((1..5).zip_map(1..5, |a, b| a + b), vec![2, 4, 6, 8]);
        assert_equal((1..5).zip_map(1..5, |a, b| a * b), vec![1, 4, 9, 16]);
    }
}
