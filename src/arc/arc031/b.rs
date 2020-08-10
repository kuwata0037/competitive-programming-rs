use proconio::{input, marker::Chars, source::auto};
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
        field: [Chars; 10]
    }

    for i in 0..field.len() {
        for j in 0..field[i].len() {
            let mut f = field.clone();
            f[i][j] = 'o';
            dfs(i, j, &mut f);
            if f.iter().all(|row| row.iter().all(|item| *item == 'x')) {
                return "YES".to_owned();
            }
        }
    }

    return "NO".to_owned();
}

fn dfs(i: usize, j: usize, field: &mut Vec<Vec<char>>) {
    if field[i][j] == 'x' {
        return;
    }

    field[i][j] = 'x';
    for &dir in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let di = (i as i32) + dir.0;
        let dj = (j as i32) + dir.1;
        if 0 <= di && di < field.len() as i32 && 0 <= dj && dj < field[i].len() as i32 {
            dfs(di as usize, dj as usize, field);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            solve(
                "xxxxxxxxxx
xoooooooxx
xxoooooxxx
xxxoooxxxx
xxxxoxxxxx
xxxxxxxxxx
xxxxoxxxxx
xxxoooxxxx
xxoooooxxx
xxxxxxxxxx
"
            ),
            "YES"
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            solve(
                "xxxxxxxxxx
xoooooooxx
xxoooooxxx
xxxoooxxxx
xxxxxxxxxx
xxxxxxxxxx
xxxxxxxxxx
xxxoooxxxx
xxoooooxxx
xxxxxxxxxx
"
            ),
            "NO"
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            solve(
                "xxxxoxxxxx
xxxxoxxxxx
xxxxoxxxxx
xxxxoxxxxx
ooooxooooo
xxxxoxxxxx
xxxxoxxxxx
xxxxoxxxxx
xxxxoxxxxx
xxxxoxxxxx
"
            ),
            "YES"
        );
    }
}
