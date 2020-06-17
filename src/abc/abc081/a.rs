use proconio::{input, marker::Chars, source::auto};
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
        chars: Chars
    };
    let chars: Vec<char> = chars;
    chars.iter().filter(|char| **char == '1').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("101"), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(solve("000"), 0);
    }
}
