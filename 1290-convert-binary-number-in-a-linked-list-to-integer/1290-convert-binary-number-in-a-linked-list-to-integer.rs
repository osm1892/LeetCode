impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        match head {
            Some(x) => match x.next {
                Some(mut next) => {
                    next.val += 2 * x.val;
                    let result = Solution::get_decimal_value(Option::Some(next.clone()));
                    next.val -= 2 * x.val;
                    result
                }
                _ => x.val,
            },
            _ => 0,
        }
    }
}