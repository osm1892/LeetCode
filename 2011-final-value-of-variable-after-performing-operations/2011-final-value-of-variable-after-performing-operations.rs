impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut x = 0;
        for i in operations {
            if i.chars().take(2).last().unwrap() == '+' {
                x += 1
            } else {
                x -= 1
            }
        }
        x
    }
}