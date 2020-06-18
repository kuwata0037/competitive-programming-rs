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
        n: usize,
        v: [(i32, i32, i32); n],
    }

    let mut v: Vec<(i32, i32, i32)> = v;
    v.insert(0, (0, 0, 0));
    let reachable = v
        .windows(2)
        .map(|elem| {
            (
                elem[1].0 - elem[0].0,
                (elem[1].1 - elem[0].1).abs(),
                (elem[1].2 - elem[0].2).abs(),
            )
        })
        .all(|(t, x, y)| x + y <= t && (t - x - y) % 2 == 0);
    if reachable {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("2\n3 1 2\n6 1 1"), "Yes");
    }

    #[test]
    fn case2() {
        assert_eq!(solve("1\n2 100 100"), "No");
    }

    #[test]
    fn case3() {
        assert_eq!(solve("2\n5 1 1\n100 1 1"), "No");
    }
}
