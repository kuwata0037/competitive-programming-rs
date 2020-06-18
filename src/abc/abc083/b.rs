use proconio::{input, source::auto};
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    println!("{}", solve(auto::AutoSource::new(stdin.lock())));
}

fn solve<T, R>(source: T) -> u32
where
    T: Into<auto::AutoSource<R>>,
    R: BufRead,
{
    input! {
        from source.into(),
        n: u32,
        a: u32,
        b: u32,
    }
    (0..=n)
        .filter(|num| {
            let sum = num
                .to_string()
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .sum();
            a <= sum && sum <= b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("20 2 5"), 84);
    }

    #[test]
    fn case2() {
        assert_eq!(solve("10 1 2"), 13);
    }

    #[test]
    fn case3() {
        assert_eq!(solve("100 4 16"), 4554);
    }
}
