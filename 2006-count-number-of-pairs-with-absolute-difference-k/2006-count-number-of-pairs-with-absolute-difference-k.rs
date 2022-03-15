impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut counter = vec![0; 201];

        for i in nums {
            counter[i as usize] += 1;
        }

        let mut result = 0;
        for i in 1..=(200 - k as usize) {
            result += counter[i] * counter[i + k as usize];
        }

        result
    }
}