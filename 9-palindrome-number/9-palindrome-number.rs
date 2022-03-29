impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut tmp = x;
        let mut rev = 0;
        while tmp > 0 {
            rev = rev * 10 + tmp % 10;
            tmp /= 10;
        }

        x == rev
    }
}