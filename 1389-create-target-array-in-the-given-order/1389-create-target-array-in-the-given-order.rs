impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let size = nums.len();
        let mut result = vec![-1; size];

        for i in 0..size {
            if result[index[i] as usize] == -1 {
                result[index[i] as usize] = nums[i];
                continue;
            }

            let mut last_swap_index = i + 1;

            for j in (index[i] as usize + 1)..size {
                if result[j] == -1 {
                    last_swap_index = j;
                }
            }

            for j in ((index[i] as usize + 1)..=last_swap_index).rev() {
                result[j] = result[j - 1];
            }
            result[index[i] as usize] = nums[i];
        }

        result
    }
}
