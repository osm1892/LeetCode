impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max_depth = 0;
        let mut depth = 0;
        let s = s.chars().collect::<Vec<char>>();
        let size = s.len();

        for i in 0..size {
            match s[i] {
                '(' => {
                    depth += 1;
                    max_depth = max_depth.max(depth);
                }
                ')' => {
                    depth -= 1;
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => match s.get(i + 1) {
                    Some(&x) => {
                        if x.is_numeric() {
                            max_depth = max_depth.max(depth + 1);
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        max_depth
    }
}