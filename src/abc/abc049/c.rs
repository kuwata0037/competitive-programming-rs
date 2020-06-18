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
        s: String,
    }
    let s: String = s.chars().rev().collect();
    let keywords: Vec<String> = ["eraser", "erase", "dreamer", "dream"]
        .iter()
        .map(|&keyword| keyword.chars().rev().collect())
        .collect();
    let mut slice = &s[..];
    while !slice.is_empty() {
        let keyword = keywords.iter().find(|&keyword| slice.starts_with(keyword));
        if let Some(word) = keyword {
            slice = &slice[word.len()..];
        } else {
            return "NO".to_string();
        }
    }
    "YES".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("erasedream"), "YES");
    }

    #[test]
    fn case2() {
        assert_eq!(solve("dreameraser"), "YES");
    }

    #[test]
    fn case3() {
        assert_eq!(solve("dreamerer"), "NO");
    }

    #[test]
    fn case4() {
        assert_eq!(solve("dreaerasem"), "NO");
    }
}
