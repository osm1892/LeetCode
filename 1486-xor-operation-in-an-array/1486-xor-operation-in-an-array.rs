impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        let mut result = start;

        for i in 1..n {
            result ^= start + i * 2;
        }

        result
    }
}