use itertools::iproduct;
use proconio::{input, source::auto};
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    println!("{}", solve(auto::AutoSource::new(stdin.lock())));
}

fn solve<T, R>(source: T) -> usize
where
    T: Into<auto::AutoSource<R>>,
    R: BufRead,
{
    input! {
        from source.into(),
        a: u32,
        b: u32,
        c: u32,
        x: u32,
    };

    iproduct!(0..=a, 0..=b, 0..=c)
        .filter(|(a, b, c)| a * 500 + b * 100 + c * 50 == x)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("2 2 2 100"), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(solve("5 1 0 150"), 0);
    }

    #[test]
    fn case3() {
        assert_eq!(solve("30 40 50 6000"), 213);
    }
}
