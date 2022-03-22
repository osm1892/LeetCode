impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allow_freq = {
            let mut arr = vec![0i8; 26];
            for c in allowed.chars() {
                arr[c as usize - 97] = 1;
            }
            arr
        };

        let mut result = 0;

        for word in words {
            let mut word_freq = vec![0i8; 26];

            for c in word.chars() {
                word_freq[c as usize - 97] = 1;
            }

            if allow_freq
                .iter()
                .zip(&word_freq)
                .filter(|&(a, b)| a - b == -1)
                .count()
                == 0
            {
                result += 1;
            }
        }

        result
    }
}