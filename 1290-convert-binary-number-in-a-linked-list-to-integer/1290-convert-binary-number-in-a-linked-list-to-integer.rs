impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut result = 0;
        let mut cursor = &head;

        loop {
            match cursor {
                Some(x) => {
                    result = result * 2 + x.val;
                    cursor = &x.next;
                },
                None => break
            }
        }

        result
    }
}