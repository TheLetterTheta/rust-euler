pub fn go(n: usize, s: String) -> u64{
    largest_product_in_series(n, s)
}

fn largest_product_in_series(n: usize, s: String) -> u64{

    let mut largest_product: u64 = 1;

    let slice_iterator = IncrementalStringSlice::new(n, s);

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
    slice_size: usize,
    increment_string: String,
    start_index: usize
}

impl IncrementalStringSlice {

    fn new(slice_size: usize, increment_string: String) -> IncrementalStringSlice {
        IncrementalStringSlice {
            slice_size: slice_size,
            increment_string: increment_string,
            start_index: 0
        }
    }
}

impl Iterator for IncrementalStringSlice {
    type Item = String;

    fn next( &mut self ) -> Option<String> {
        if (self.start_index + self.slice_size) > self.increment_string.len() {
            return None;
        }

        let ret: String = self.increment_string.chars().skip(self.start_index).take(self.slice_size).collect();
        self.start_index += 1;

        Some(ret)
    }
}
