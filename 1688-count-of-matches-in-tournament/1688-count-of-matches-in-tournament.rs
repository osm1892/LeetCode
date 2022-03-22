impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut n = n;
        let mut result = 0;

        while n > 1 {
            let tmp = (n as f64) / 2.0;
            n = tmp.ceil() as i32;
            result += tmp.floor() as i32;
        }

        result
    }
}