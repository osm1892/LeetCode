use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let root = root.as_deref().unwrap().borrow();
        let mut sum = 0;

        if low <= root.val && root.val <= high {
            sum += root.val;
        }

        if !root.left.is_none() && root.val >= low {
            sum += Solution::range_sum_bst(root.left.clone(), low, high);
        }

        if !root.right.is_none() && root.val <= high {
            sum += Solution::range_sum_bst(root.right.clone(), low, high);
        }

        sum
    }
}