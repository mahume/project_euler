/// Problem: [Multiples of 3 or 5](https://projecteuler.net/problem=1)
///
/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
/// Find the sum of all the multiples of 3 or 5 below 1000.

fn main() {
    println!("Hello, Euler!")
}

fn solve() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mock_range = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let multiples = vec![3, 5];
        let multiples_of = vec![3, 5, 6, 9];

        assert_eq!(solve(mock_range), 23);
    }
}
