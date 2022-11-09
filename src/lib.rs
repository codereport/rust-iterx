// #[cfg(test)]
// use std::ops::Add;

// pub fn option_lift<F, I, O>(f: F) -> impl Fn(I, I) -> Option<O>
// where
//     F: Fn(I, I) -> O,
// {
//     return |*acc, x| {
//         *acc = f(*acc, x);
//         Some(*acc)
//     };
// }

// pub trait Iterscans : Iterator {
//     fn scan_while(self, f: F) {}
//     fn scan() {}
//     fn prescan_while() {}
//     fn pres
// }

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_scan_not_what_i_want() {
        let input = vec![1, 1, 1];
        let expected = vec![1, 2, 3];
        assert_eq!(
            input
                .into_iter()
                // .scan(0, &option_lift(&Add::add))
                .scan(0, |acc, x| {
                    *acc += x;
                    Some(*acc)
                })
                .collect::<Vec<_>>(),
            expected
        );
    }
}
