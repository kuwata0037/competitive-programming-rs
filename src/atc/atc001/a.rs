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
        h: usize,
        _w: usize,
        mut field: [Chars; h]
    }
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if field[i][j] == 's' {
                return if dfs(i, j, &mut field) {
                    "Yes".to_owned()
                } else {
                    "No".to_owned()
                };
            }
        }
    }

    return "No".to_owned();
}

fn dfs(i: usize, j: usize, field: &mut Vec<Vec<char>>) -> bool {
    if field[i][j] == 'g' {
        return true;
    }
    if field[i][j] == '#' {
        return false;
    }
    field[i][j] = '#';
    for &dir in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let di = (i as i32) + dir.0;
        let dj = (j as i32) + dir.1;
        if 0 <= di && di < field.len() as i32 && 0 <= dj && dj < field[i].len() as i32 {
            if dfs(di as usize, dj as usize, field) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            solve(
                "4 5
s####
....#
#####
#...g
"
            ),
            "No"
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            solve(
                "4 4
...s
....
....
.g..
"
            ),
            "Yes"
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            solve(
                "10 10
s.........
#########.
#.......#.
#..####.#.
##....#.#.
#####.#.#.
g.#.#.#.#.
#.#.#.#.#.
###.#.#.#.
#.....#...
"
            ),
            "No"
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            solve(
                "10 10
s.........
#########.
#.......#.
#..####.#.
##....#.#.
#####.#.#.
g.#.#.#.#.
#.#.#.#.#.
#.#.#.#.#.
#.....#...
"
            ),
            "Yes"
        )
    }

    #[test]
    fn case5() {
        assert_eq!(
            solve(
                "1 10
s..####..g
"
            ),
            "No"
        )
    }
}
