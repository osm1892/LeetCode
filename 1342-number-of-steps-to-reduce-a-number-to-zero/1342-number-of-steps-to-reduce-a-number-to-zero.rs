impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut num = num;
        let mut result = -1;
        
        if num == 0 {
            return 0
        }
        
        while num > 0 {
            result += (num & 1) + 1;
            num >>= 1;
        }
        result
    }
}