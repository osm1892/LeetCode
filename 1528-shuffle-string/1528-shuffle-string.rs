impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut result = vec![' '; s.len()];
        let mut i = 0;

        for c in s.chars() {
            result[indices[i] as usize] = c;
            i += 1;
        }

        result.into_iter().collect()
    }
}