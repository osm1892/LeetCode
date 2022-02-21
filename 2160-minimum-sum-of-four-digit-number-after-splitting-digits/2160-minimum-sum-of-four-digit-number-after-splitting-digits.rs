impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut tmp = num.to_string().bytes().map(|x| x as i32 - 48).collect::<Vec<i32>>();
        tmp.sort_unstable();

        tmp[0] * 10 + tmp[2] + tmp[1] * 10 + tmp[3]
    }
}
