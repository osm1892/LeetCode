struct OrderedStream {
    stream: Vec<Option<String>>,
    index: usize,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream {
            stream: vec![Option::None; (n + 1) as usize],
            index: 1,
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.stream[id_key as usize] = Some(value);

        let mut result: Vec<String> = Vec::new();

        loop {
            match self.stream.get(self.index) {
                Some(Some(x)) => {
                    result.push(x.clone());
                    self.index += 1;
                }
                _ => break,
            };
        }

        result
    }
}