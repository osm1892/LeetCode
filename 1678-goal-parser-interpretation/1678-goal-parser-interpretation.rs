impl Solution {
    pub fn interpret(command: String) -> String {
        let mut result: Vec<char> = Vec::new();

        let mut is_paren = false;
        for c in command.chars() {
            match c {
                'G' => {result.push('G');},
                '(' => {is_paren = true;},
                'a' => {
                    is_paren = false;
                    result.push('a');
                    result.push('l');
                },
                ')' => {
                    if is_paren {
                        result.push('o');
                    }
                },
                _ => (),
            };
        }

        result.into_iter().collect()
    }
}