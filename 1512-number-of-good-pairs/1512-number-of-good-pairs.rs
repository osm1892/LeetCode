impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut counter = vec![0; 101];
        for i in nums {
            counter[i as usize] += 1;
        }
        let mut result = 0;
        for i in counter {
            result += i * (i - 1) / 2;
        }
        result
    }
}
