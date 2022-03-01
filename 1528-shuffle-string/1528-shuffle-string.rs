impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut result = vec![' '; s.len()];

        for i in s.chars().zip(indices) {
            result[i.1 as usize] = i.0;
        }

        result.into_iter().collect()
    }
}