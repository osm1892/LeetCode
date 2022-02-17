impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut counter = vec![0; 101];
        nums.iter().for_each(|&x| counter[x as usize] += 1);
        counter
            .into_iter()
            .reduce(|acc, x| acc + x * (x - 1) / 2)
            .unwrap()
    }
}
