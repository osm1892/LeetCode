impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let (start, end): (&str, &str) = {
            let mut tmp = s.split(":");
            (tmp.next().unwrap(), tmp.next().unwrap())
        };

        let start_col_index = start.find(char::is_numeric).unwrap();
        let end_col_index = end.find(char::is_numeric).unwrap();

        let start = (
            &start[..start_col_index],
            &start[start_col_index..].parse::<i32>().unwrap(),
        );
        let end = (
            &end[..end_col_index],
            &end[end_col_index..].parse::<i32>().unwrap(),
        );

        let mut start_row = 0usize;
        for i in start.0.bytes() {
            start_row *= 26;
            start_row += i as usize - 'A' as usize;
        }

        let mut end_row = 0usize;
        for i in end.0.bytes() {
            end_row *= 26;
            end_row += i as usize - 'A' as usize;
        }

        let mut result: Vec<String> = Vec::new();

        for row in start_row..=end_row {
            for col in *start.1..=*end.1 {
                result.push(format!("{}{}", num_to_alpha(row), col));
            }
        }

        result
    }
}

fn num_to_alpha(n: usize) -> String {
    let mut result: Vec<char> = Vec::new();
    let mut n = n;

    if n == 0 {
        result.push('A');
    }

    while n != 0 {
        result.push(((n % 26) as u8 + 65) as char);
        n /= 26;
    }

    result.into_iter().rev().collect()
}