// Nicholas Dolan
// 2018-11-13
// Project Euler #2
pub fn go(n: u32) -> u64 {

    let mut sum: u64 = 0;

    for i in fibonacci()
        .filter(|i| i % 2 == 0)
        .take(n as usize){
        sum += i as u64;
    }

    return sum;
}

struct Fibonacci {
    curr: u32,
    next: u32
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next( &mut self ) -> Option<u32> {
        if self.next >= 4000000 {
            return None
        }

        let t = self.curr + self.next;
        self.curr = self.next;
        self.next = t;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1}
}
