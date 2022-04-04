impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let size = nums.len();

        nums[size - 1] * nums[size - 2] - nums[0] * nums[1]
    }
}