impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let size = rings.len();
        let mut rods = [0u8; 10];
        let rings: Vec<char> = rings.chars().collect();

        for i in (0..size).step_by(2) {
            let mask = match rings[i] {
                'R' => 1,
                'G' => 2,
                'B' => 4,
                _ => 0,
            };
            let index = rings[i + 1] as usize - 48;

            rods[index] |= mask;
        }

        rods.iter().filter(|&&x| x == 7).count() as i32
    }
}