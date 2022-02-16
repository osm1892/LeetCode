impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences.iter().map(|x| (x.chars().filter(|&c| c == ' ').count() + 1) as i32).max().unwrap()
    }
}