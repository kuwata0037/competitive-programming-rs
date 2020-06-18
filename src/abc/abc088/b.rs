use proconio::{input, source::auto};
use std::cmp::Reverse;
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
        n: usize,
        v: [u32; n],
    }
    let mut v: Vec<u32> = v;
    if n % 2 != 0 {
        v.push(0);
    }
    v.sort_by_key(|&key| Reverse(key));
    v.chunks(2).map(|chunk| chunk[0] - chunk[1]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("2 3 1"), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(solve("3 2 7 4"), 5);
    }

    #[test]
    fn case3() {
        assert_eq!(solve("4 20 18 2 18"), 18);
    }
}
