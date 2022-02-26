impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::<i32>::new();
        for i in (0..nums.len()).step_by(2) {
            result.extend(vec![nums[i + 1]; nums[i] as usize]);
        }
        result
    }
}
