macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

macro_rules! stdin {
    () => {{
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }};
}

#[allow(dead_code)]
fn main() {
    println!("{}", solve(&stdin!()));
}

fn enable_traveling(v: &[(i32, i32, i32)]) -> bool {
    v.windows(2).all(|elements| {
        let (t1, x1, y1) = elements[0];
        let (t2, x2, y2) = elements[1];
        let delta_t = t2 - t1;
        let shortest_distance = (x2 - x1).abs() + (y2 - y1).abs();
        if delta_t < shortest_distance {
            return false;
        }
        if (delta_t - shortest_distance).trailing_zeros() == 0 {
            return false;
        }
        true
    })
}

fn solve(src: &str) -> String {
    input!(source = src, n: usize, v: [(i32, i32, i32); n]);
    let mut v: Vec<_> = v;
    v.insert(0, (0, 0, 0));
    if enable_traveling(&v) {
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
        assert_eq!(solve("1\n2 100 100"), "No");
        assert_eq!(solve("2\n5 1 1\n100 1 1"), "No");
    }
}
