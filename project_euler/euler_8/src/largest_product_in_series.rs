pub fn go(n: usize, s: String) -> u64{
    largest_product_in_series(n, s)
}

fn largest_product_in_series(n: usize, s: String) -> u64{

    let mut largest_product: u64 = 1;

    let slice_iterator = IncrementalStringSlice {
        len : n,
        s: s,
        start_index: 0
    };

    for i in slice_iterator {
        let mut curr_largest_product = 1;
        for j in i.chars().map(|k| k.to_digit(10).unwrap()) {
            curr_largest_product *= j as u64;
        }
        if curr_largest_product > largest_product {
            largest_product = curr_largest_product
        }
    }

    largest_product
}

struct IncrementalStringSlice {
    len: usize,
    s: String,
    start_index: usize
}

impl Iterator for IncrementalStringSlice {
    type Item = String;

    fn next( &mut self ) -> Option<String> {
        if (self.start_index + self.len) > self.s.len() {
            return None;
        }

        let ret: String = self.s.chars().skip(self.start_index).take(self.len).collect();
        self.start_index += 1;

        Some(ret)
    }
}
