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
    input! {from source.into(), a: u32, b: u32};
    if a * b % 2 == 0 {
        "Even".to_string()
    } else {
        "Odd".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("3 4"), "Even");
    }

    #[test]
    fn case2() {
        assert_eq!(solve("1 21"), "Odd");
    }
}
