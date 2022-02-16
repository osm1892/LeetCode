impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut split = address.split(".");
        let mut result: Vec<char> = Vec::new();

        for i in 0..3 {
            result.extend(split.next().unwrap().chars());
            result.extend("[.]".chars());
        }
        result.extend(split.next().unwrap().chars());

        result.iter().collect()
    }
}
