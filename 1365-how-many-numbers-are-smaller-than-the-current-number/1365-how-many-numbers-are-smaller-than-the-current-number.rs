impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut counter: Vec<i32> = vec![0; 102];

        for &num in &nums {
            counter[num as usize + 1] += 1;
        }

        for i in 1..counter.len() {
            counter[i] += counter[i - 1];
        }
        nums.into_iter().map(|x| counter[x as usize]).collect()
    }
}