impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let size = encoded.len();
        let mut decoded: Vec<i32> = Vec::with_capacity(size + 1);
        decoded.push(first);

        for i in 0..size {
            decoded.push(decoded[i] ^ encoded[i]);
        }

        decoded
    }
}