use proconio::{input, source, source::once};
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    println!("{}", solve(once::OnceSource::new(stdin.lock())));
}

fn solve<T, R>(source: T) -> String
where
    T: source::Source<R>,
    R: BufRead,
{
    input! {from source, a: u32, b: u32};
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
        assert_eq!(solve(once::OnceSource::from("3 4")), "Even");
    }

    #[test]
    fn case2() {
        assert_eq!(solve(once::OnceSource::from("1 21")), "Odd");
    }
}
