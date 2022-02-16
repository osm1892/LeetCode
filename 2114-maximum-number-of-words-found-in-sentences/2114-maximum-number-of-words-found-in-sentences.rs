impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut result = 0;
        for line in sentences {
            let mut counter = 0;
            for c in line.chars() {
                counter += if c == ' ' { 1 } else { 0 }
            }
            result = result.max(counter + 1);
        }
        result
    }
}