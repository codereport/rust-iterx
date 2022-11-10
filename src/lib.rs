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
//     fn scan_while(self, f: F) {}
//     fn scan() {}
//     fn prescan_while() {}
//     fn pres
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_not_what_i_want() {
        let input = vec![1, 1, 1];
        let expected = vec![1, 2, 3];
        assert_eq!(
            input
                .into_iter()
                .scan(0, &option_lift(&|a, b| *a + b))
                .collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn test_w_combinator() {
        assert_eq!(w(&Add::add)(1), 2);
    }
}
