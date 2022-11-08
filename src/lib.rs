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
                .scan(0, |acc, x| {
                    *acc += x;
                    Some(*acc)
                })
                .collect::<Vec<_>>(),
            expected
        );
    }
}
