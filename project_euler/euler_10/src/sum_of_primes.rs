use num::{Num, ToPrimitive, FromPrimitive, Unsigned, Integer};

pub struct PrimeVect<T> where T: Num + Unsigned + Integer + Copy + ToPrimitive {
    primes: Vec<T>,
    _largest_size: T,
    index: T
}

impl<T> PrimeVect<T> where T: Num + Unsigned + Integer + Copy + ToPrimitive + FromPrimitive {
    pub fn new() -> PrimeVect<T>{
        PrimeVect {
            primes: vec![T::from_u8(2).unwrap(), T::from_u8(3).unwrap()],
            _largest_size: T::from_u8(4).unwrap(),
            index: T::from_u8(0).unwrap()
        }
    }

    pub fn new_siev_with_size(initial_size: T) -> PrimeVect<T> {
        PrimeVect {
            primes: vec![T::from_u8(2).unwrap(), T::from_u8(3).unwrap()],
            _largest_size: T::from_u8(4).unwrap(),
            index: T::from_u8(0).unwrap()
        }
    }

    fn resize(&mut self) {
        let step = self._largest_size.to_usize().unwrap();
        let mut collected_primes = Vec::new();
        for i in 1..step {
            let start = (step * i) + 1;
            let size = step / 2;

            let added_primes = &mut (0..size)
                .map(|j| T::from_usize(start + (j * 2)).unwrap())
                .filter(|j| self.primes.iter().all(|p|  ! j.is_multiple_of(p) ))
                .collect();
            collected_primes.append(added_primes);
        }

        self.primes.append(&mut collected_primes);
        self._largest_size = self._largest_size * T::from_u8(2).unwrap();
    }
}

impl<T> Iterator for PrimeVect<T> where T: Num + Unsigned + Integer + Copy + ToPrimitive + FromPrimitive {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>{
        let curr_index = self.index.to_usize().unwrap();
        if curr_index == self.primes.len() {
            self.resize();
        }

        self.index = self.index + T::one();
        Some(self.primes[curr_index])
    }
}
