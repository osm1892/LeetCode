impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        for x in 0..2 {
            for y in 0..2 {
                if edges[0][x] == edges[1][y] {
                    return edges[0][x];
                }
            }
        }
        0
    }
}