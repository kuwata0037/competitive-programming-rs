use proconio::{input, source::auto};
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    println!("{}", solve(auto::AutoSource::new(stdin.lock())));
}

fn solve<T, R>(source: T) -> String
where
    T: Into<auto::AutoSource<R>>,
    R: BufRead,
{
    input! {
        from source.into(),
        n: u32,
        y: u32,
    }

    for i in 0..=n {
        for j in 0..=n - i {
            if 10000 * i + 5000 * j + 1000 * (n - i - j) == y {
                return format!("{} {} {}", i, j, n - i - j);
            }
        }
    }

    "-1 -1 -1".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("9 45000"), "0 9 0");
    }

    #[test]
    fn case2() {
        assert_eq!(solve("20 196000"), "-1 -1 -1");
    }

    #[test]
    fn case3() {
        assert_eq!(solve("1000 1234000"), "2 54 944");
    }

    #[test]
    fn case4() {
        assert_eq!(solve("2000 20000000"), "2000 0 0");
    }
}
