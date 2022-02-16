impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut split = address.split(".");
        format!(
            "{}[.]{}[.]{}[.]{}",
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap()
        )
    }
}
