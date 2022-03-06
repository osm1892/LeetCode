impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut counter = 0;
        let mut result = 0;
        for c in s.chars() {
            if c == 'R' {
                counter += 1;
            } else {
                counter -= 1;
            }
            if counter == 0 {
                result += 1;
            }
        }

        result
    }
}