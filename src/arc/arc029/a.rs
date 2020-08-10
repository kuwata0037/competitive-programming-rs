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
        mut t: [u32; n],
    }

    t.sort_by_key(|key| Reverse(*key));
    let mut a = 0;
    let mut b = 0;
    for v in &t {
        if a <= b {
            a += v;
        } else {
            b += v;
        }
    }
    a.max(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("4\n4\n6\n7\n10"), 14);
    }

    #[test]
    fn case2() {
        assert_eq!(solve("3\n1\n2\n4"), 4);
    }

    #[test]
    fn case3() {
        assert_eq!(solve("1\n29"), 29);
    }
}
