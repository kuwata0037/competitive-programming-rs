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
        k: i32,
        s: i32,
    }

    let v: Vec<i32> = (0..=k).collect();
    iproduct!(&v, &v)
        .filter(|(x, y)| {
            let rest = s - *x - *y;
            0 <= rest && rest <= k
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("2 2"), 6);
    }

    #[test]
    fn case2() {
        assert_eq!(solve("5 15"), 1)
    }
}
