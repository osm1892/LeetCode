use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut jewel_set: HashSet<char> = HashSet::new();
        let mut tmp: bool = false;
        jewels.chars().for_each(|x| tmp = jewel_set.insert(x));
        let mut result = 0;
        stones.chars().for_each(|x| result += jewel_set.contains(&x) as i32);
        result
    }
}