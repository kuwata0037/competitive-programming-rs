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

fn solve(src: &str) -> String {
    let src = src.trim_right();
    let src = src.split("eraser").collect::<String>();
    let src = src.split("erase").collect::<String>();
    let src = src.split("dreamer").collect::<String>();
    let src = src.split("dream").collect::<String>();

    if src.is_empty() { "YES" } else { "NO" }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve("erasedream"), "YES");
        assert_eq!(solve("dreameraser"), "YES");
        assert_eq!(solve("dreamerer"), "NO");
    }
}
