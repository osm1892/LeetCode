impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let size = arr.len();
        let mut sums = vec![0; size];
        sums[0] = arr[0];

        for i in 1..size {
            sums[i] = arr[i] + sums[i - 1];
        }

        let mut result = 0;
        for step in (1..=size).step_by(2) {
            for i in 0..=(size - step) {
                result += sums[i + step - 1] - sums[i] + arr[i];
            }
        }

        result
    }
}