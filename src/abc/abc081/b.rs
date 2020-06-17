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
        n: usize,
        a: [u32; n],
    }
    let a: Vec<u32> = a;
    a.into_iter()
        .map(|value| value.trailing_zeros())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("3\n8 12 40"), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(solve("4\n5 6 8 10"), 0);
    }

    #[test]
    fn case3() {
        assert_eq!(
            solve("6\n382253568 723152896 37802240 379425024 404894720 471526144"),
            8
        );
    }
}
