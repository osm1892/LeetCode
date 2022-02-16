impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(2 * n as usize);
        for i in 0..n as usize {
            result.push(nums[i]);
            result.push(nums[n as usize + i]);
        }
        result
    }
}