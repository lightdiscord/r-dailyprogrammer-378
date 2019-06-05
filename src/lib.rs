#![feature(drain_filter)]

pub fn havel_hakimi(mut sequence: Vec<usize>) -> bool {
    loop {
        sequence.drain_filter(|x| *x == 0).for_each(drop);

        if sequence.is_empty() {
            return true;
        }

        sequence.sort();
        sequence.reverse();

        let n = sequence.remove(0);

        if n > sequence.len() {
            return false;
        }

        for answer in &mut sequence[0..n] {
            *answer -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_challenges() {
        assert_eq!(false, havel_hakimi(vec![15, 18, 6, 13, 12, 4, 4, 14, 1, 6, 18, 2, 6, 16, 0, 9, 10, 7, 12, 3]));
        assert_eq!(false, havel_hakimi(vec![6, 0, 10, 10, 10, 5, 8, 3, 0, 14, 16, 2, 13, 1, 2, 13, 6, 15, 5, 1]));
        assert_eq!(false, havel_hakimi(vec![5, 3, 0, 2, 6, 2, 0, 7, 2, 5]));
        assert_eq!(false, havel_hakimi(vec![4, 2, 0, 1, 5, 0]));
        assert_eq!(false, havel_hakimi(vec![2, 2, 0]));
        assert_eq!(false, havel_hakimi(vec![3, 2, 1]));
        assert_eq!(false, havel_hakimi(vec![1]));

        assert_eq!(true, havel_hakimi(vec![16, 9, 9, 15, 9, 7, 9, 11, 17, 11, 4, 9, 12, 14, 14, 12, 17, 0, 3, 16]));
        assert_eq!(true, havel_hakimi(vec![14, 10, 17, 13, 4, 8, 6, 7, 13, 13, 17, 18, 8, 17, 2, 14, 6, 4, 7, 12]));
        assert_eq!(true, havel_hakimi(vec![3, 1, 2, 3, 1, 0]));
        assert_eq!(true, havel_hakimi(vec![1, 1]));
        assert_eq!(true, havel_hakimi(vec![]));
    }
}
