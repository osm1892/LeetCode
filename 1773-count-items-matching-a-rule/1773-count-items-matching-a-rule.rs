impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let key_index = match &rule_key as &str {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => 3,
        };

        items.into_iter().map(|x| (x[key_index] == rule_value) as i32).sum()
    }
}