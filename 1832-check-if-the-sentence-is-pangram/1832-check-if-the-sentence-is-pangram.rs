impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        if sentence.len() < 26 {
            return false;
        }

        let mut counter = [false; 26];

        for c in sentence.chars() {
            counter[c as usize - 97] = true;
        }

        counter.iter().filter(|&&x| !x).count() == 0
    }
}