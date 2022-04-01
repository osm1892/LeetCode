use std::collections::VecDeque;
use std::ops::Add;

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut result = 0;

        let size = {
            let mut size = 0;
            let mut head = &head;
            loop {
                head = match head {
                    Some(ref x) => {
                        size += 1;
                        &x.next
                    }
                    None => {
                        break;
                    }
                };
            }
            size
        };

        let mut stack: VecDeque<i32> = VecDeque::new();

        stack.push_back(match head {
            Some(ref x) => x.val,
            None => 0,
        });

        let mut head = &head;
        for i in 1..size {
            head = match head {
                Some(ref x) => &x.next,
                None => &None,
            };
            if i < size / 2 {
                match head {
                    Some(ref x) => stack.push_back(x.val),
                    None => {}
                }
            } else {
                result = result.max(stack.pop_back().unwrap().add(match head {
                    Some(ref x) => x.val,
                    None => 0,
                }));
            }
        }

        result
    }
}