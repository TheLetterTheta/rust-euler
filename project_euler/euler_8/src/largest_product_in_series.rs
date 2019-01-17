use std::vec::IntoIter;

pub fn go(n: usize, s: &str) -> u64{
    largest_product_in_series(n, s)
}

fn largest_product_in_series(n: usize, s: &str) -> u64{

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

struct IncrementalStringSlice<'a> {
    slice_size: usize,
    increment_string: &'a str
}

impl<'a> IncrementalStringSlice<'a> {
    fn new(slice_size: usize, increment_string: &'a str) -> IncrementalStringSlice{
        IncrementalStringSlice {
            slice_size: slice_size,
            increment_string: increment_string
        }
    }
}

impl<'a> IntoIterator for IncrementalStringSlice<'a> {
    type Item = &'a str;
    type IntoIter = IntoIter<&'a str>;

    fn into_iter(self) -> Self::IntoIter {
        let end_of_iter = self.increment_string.len() - self.slice_size + 1;
        (0..end_of_iter)
            .map(|index|
                    &self.increment_string[index..(index + self.slice_size)]
            )
            .collect::<Vec<&str>>()
            .into_iter()
    }
}
