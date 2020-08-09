use itertools::iproduct;
use proconio::{input, source::auto};
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    println!("{}", solve(auto::AutoSource::new(stdin.lock())));
}

fn solve<T, R>(source: T) -> f64
where
    T: Into<auto::AutoSource<R>>,
    R: BufRead,
{
    input! {
        from source.into(),
        n: usize,
        p: [(f64, f64); n],
    }

    let p: Vec<(f64, f64)> = p;
    itertools::iproduct!(&p, &p)
        .map(|(p1, p2)| (p1.0 - p2.0).hypot(p1.1 - p2.1))
        .max_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("3\n1 1\n2 4\n4 3"), 3.605551275463989);
    }

    #[test]
    fn case2() {
        assert_eq!(
            solve("10\n1 8\n4 0\n3 7\n2 4\n5 9\n9 1\n6 2\n0 2\n8 6\n7 8"),
            10.63014581273465
        );
    }

    #[test]
    fn case3() {
        assert_eq!(solve("4\n0 0\n0 100\n100 0\n100 100"), 141.4213562373095);
    }

    #[test]
    fn case4() {
        assert_eq!(solve("5\n3 0\n1 0\n0 0\n4 0\n2 0"), 4.000000);
    }

    #[test]
    fn case5() {
        assert_eq!(solve("4\n2 2\n0 0\n1 1\n3 3"), 4.242640687119285);
    }
}
