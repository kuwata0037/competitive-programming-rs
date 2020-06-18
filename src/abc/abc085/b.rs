use proconio::{input, source::auto};
use std::collections::HashSet;
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
        n: usize,
        d: [u32; n],
    }
    let d: Vec<u32> = d;
    d.into_iter().collect::<HashSet<_>>().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("4 10 8 8 6",), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(solve("3 15 15 15"), 1);
    }

    #[test]
    fn case3() {
        assert_eq!(solve("7 50 30 50 100 50 80 30"), 4);
    }
}
