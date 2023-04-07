use std::error;
use std::ops::Range;

/// Problem: [Multiples of 3 or 5](https://projecteuler.net/problem=1)
///
/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
/// Find the sum of all the multiples of 3 or 5 below 1000.

fn main() -> Result<(), Box<dyn error::Error>> {
    let natural_numbers_list = 0..1000;
    let multiples = vec![3, 5];

    let r = Ranger::new(natural_numbers_list, multiples);

    println!("Sum of multiples: {}", r.sum_of_multiples());

    Ok(())
}

struct Ranger {
    natural_numbers: Range<i32>,
    multiples: Vec<i32>,
}

impl Ranger {
    fn new(natural_numbers: Range<i32>, multiples: Vec<i32>) -> Self {
        Self {
            natural_numbers,
            multiples,
        }
    }

    fn multiples(&self) -> Vec<i32> {
        self.natural_numbers
            .clone()
            .filter(|nn| *nn != 0 && self.multiples.iter().any(|m| nn % m == 0))
            .collect()
    }

    fn sum_of_multiples(&self) -> i32 {
        self.multiples().iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_multiples() {
        let r = Ranger::new(0..10, vec![3, 5]);

        assert_eq!(r.multiples(), vec![3, 5, 6, 9]);
    }

    #[test]
    fn sum_of_multiples() {
        let r = Ranger::new(0..10, vec![3, 5]);

        assert_eq!(r.sum_of_multiples(), 23);
    }
}
