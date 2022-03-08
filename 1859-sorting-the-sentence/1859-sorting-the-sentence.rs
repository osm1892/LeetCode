impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut sorted = s.split_whitespace().collect::<Vec<&str>>();
        sorted.sort_unstable_by_key(|&x| Solution::get_index_from_sentence(x));
        sorted
            .into_iter()
            .map(|x: &str| x.replace(|c: char| c.is_numeric(), " "))
            .collect::<String>()
            .trim_end()
            .to_string()
    }

    fn get_index_from_sentence(s: &str) -> usize {
        let bytes = s.as_bytes();
        let size = s.len();

        let mut index = size - 1;
        let mut digit = 1;
        let mut result = 0;

        while bytes[index].is_ascii_digit() {
            result += (bytes[index] as usize + 48) * digit;
            index -= 1;
            digit *= 10;
        }

        result
    }
}