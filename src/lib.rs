#![feature(iterator_step_by)]

extern crate num_integer;

pub mod problem_one {
    pub fn multiples_of_3_and_5(n: u32) -> u32 {
        (0..n)
            .into_iter()
            .filter(|el| (el % 3) == 0 || (el % 5) == 0)
            .sum()
    }
}

pub mod problem_two {
    fn allocate_fibo(n: u32) -> Vec<u32> {
        let mut vec = vec![1, 2];
        while *vec.last().unwrap() <= n {
            let len = vec.len();
            let new_num = vec[len - 2] + vec[len - 1];
            vec.push(new_num);
        }
        vec
    }

    fn calculate_sum_even_fibo(vec: &[u32]) -> u32 {
        vec.iter()
            .enumerate()
            .filter(|(index, _)| index % 3 == 1)
            .map(|(_, element)| element)
            .sum()
    }

    pub fn problem_two_script(n: u32) -> u32 {
        calculate_sum_even_fibo(&allocate_fibo(n))
    }
}

pub mod problem_three {
    use std::cell::Cell;

    pub fn largest_prime_divisor(n: u64) -> u64 {
        // Need to use cell since `take_while` and `filter` borrows `n` immutably
        // but `for_each` mutably
        let n = Cell::new(n);
        let mut div = 0;
        (2..=n.get())
            .take_while(|_| n.get() > 1)
            .filter(|a| n.get() % a == 0)
            .for_each(|a| {
                while n.get() % a == 0 {
                    n.set(n.get() / a);
                }
                div = a;
            });
        div
    }
}

pub mod problem_four {
    struct ThreeDigitPair((u32, u32));

    impl ThreeDigitPair {
        fn new(a: u32, b: u32) -> Option<Self> {
            if num_digits(a) == 3 && num_digits(b) == 3 {
                Some(ThreeDigitPair((a, b)))
            } else {
                None
            }
        }
    }
    impl Iterator for ThreeDigitPair {
        type Item = (u32, u32);
        fn next(&mut self) -> Option<(u32, u32)> {
            match self.0 {
                (a, b) if a == 1000 && b == 1000 => None,
                (a, b) if b == 1000 => {
                    (self.0).0 += 1;
                    (self.0).1 = 0;
                    Some((a, b))
                }
                (a, b) => {
                    (self.0).1 += 1;
                    Some((a, b))
                }
            }
        }
    }

    fn is_palindrome(n: u32) -> bool {
        let n_str = n.to_string();
        let bytes = n_str.as_bytes();
        let len = bytes.len();
        let last_index = len - 1;
        let mid_index = len / 2;
        bytes
            .into_iter()
            .enumerate()
            .take_while(|tuple| tuple.0 <= mid_index)
            .all(|tuple| tuple.1 == &bytes[last_index - tuple.0])
    }

    pub fn num_digits(n: u32) -> usize {
        if n == 0 {
            1
        } else {
            ((n as f32).log10() + 1.).floor() as usize
        }
    }

    pub fn problem_four_script() -> u32 {
        let new_pair = ThreeDigitPair::new(100, 100).unwrap();
        new_pair
            .filter_map(|pair| {
                let compt = pair.0 * pair.1;
                if is_palindrome(compt) {
                    Some(compt)
                } else {
                    None
                }
            })
            .max()
            .unwrap()
    }
}

pub mod problem_five {
    use num_integer::lcm;

    pub fn smallest_multiple(n: u32) -> u32 {
        (1..=n).fold(1, lcm)
    }
}

pub mod problem_six {
    pub fn sum_square_difference(n: u32) -> u32 {
        (1..=n).sum::<u32>().pow(2) - (1..=n).map(|n| n.pow(2)).sum::<u32>()
    }
}

pub mod problem_seven {
    pub fn nth_prime(n: usize) -> u64 {
        (2..).filter(|u| is_prime(*u)).take(n).last().unwrap()
    }

    fn is_prime(n: u64) -> bool {
        !(2..n).any(|u| n % u == 0)
    }
}

pub mod problem_eight {
    pub fn adjacent_products(num_str: &str, n: usize) -> u64 {
        let chars_vec: Vec<u64> = num_str
            .chars()
            .map(|c| u64::from(c.to_digit(10).unwrap()))
            .collect();
        let len = chars_vec.len();
        (0..=len - n)
            .map(|u| chars_vec[u..u + n].into_iter().product::<u64>())
            .max()
            .unwrap()
    }
}

pub mod problem_nine {
    use problem_four::num_digits;

    struct AtMostThreeDigitPair((u32, u32));

    impl AtMostThreeDigitPair {
        fn new(a: u32, b: u32) -> Option<Self> {
            if num_digits(a) <= 3 && num_digits(b) <= 3 && a < b {
                Some(AtMostThreeDigitPair((a, b)))
            } else {
                None
            }
        }
    }
    impl Iterator for AtMostThreeDigitPair {
        type Item = (u32, u32);
        fn next(&mut self) -> Option<(u32, u32)> {
            match self.0 {
                (a, b) if a == 999 && b == 1000 => None,
                (a, b) if a + 1 == b => {
                    (self.0).0 = 1;
                    (self.0).1 += 1;
                    Some((a, b))
                }
                (a, b) => {
                    (self.0).0 += 1;
                    Some((a, b))
                }
            }
        }
    }
    pub fn pythagorean_triplet(sum: u32) -> Vec<(u32, u32)> {
        let new_pair = AtMostThreeDigitPair::new(0, 1).unwrap();
        new_pair
            .take_while(|pair| pair.1 <= sum)
            .filter_map(|pair| {
                let sqrt = f64::from(pair.0.pow(2) + pair.1.pow(2)).sqrt();
                if sqrt.fract() == 0. && pair.0 + pair.1 + sqrt as u32 == sum {
                    Some((pair.0, pair.1))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub mod problem_ten {
    use std::cell::Cell;

    pub fn sum_of_primes(n: u64) -> u64 {
        let sieve: Vec<Cell<bool>> = vec![Cell::new(true); (n - 2) as usize];

        (2..n)
            .filter(|u| *u <= (n as f64).sqrt() as u64)
            .filter(|u| sieve[(u - 2) as usize].get())
            .for_each(|u| {
                (u.pow(2)..n)
                    .step_by(u as usize)
                    .for_each(|u| sieve[(u - 2) as usize].set(false))
            });
        sieve
            .into_iter()
            .enumerate()
            .filter(|pair| pair.1.get())
            .map(|pair| (pair.0 + 2) as u64)
            .sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use euler::*;

    const PROBLEM_8_STR: &str = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

    #[test]
    fn problem_one() {
        assert_eq!(problem_one::multiples_of_3_and_5(1000), 233168);
    }
    #[test]
    fn problem_two() {
        assert_eq!(problem_two::problem_two_script(4_000_000), 4613732);
    }
    #[test]
    fn problem_third() {
        assert_eq!(problem_three::largest_prime_divisor(600851475143), 6857);
    }
    #[test]
    fn problem_four() {
        assert_eq!(problem_four::problem_four_script(), 906609);
    }
    #[test]
    fn problem_five() {
        assert_eq!(problem_five::smallest_multiple(20), 232792560);
    }
    #[test]
    fn problem_six() {
        assert_eq!(problem_six::sum_square_difference(100), 25164150);
    }
    #[test]
    #[ignore]
    fn problem_seven() {
        assert_eq!(problem_seven::nth_prime(10_001), 104743);
    }
    #[test]
    fn problem_eight() {
        assert_eq!(
            problem_eight::adjacent_products(PROBLEM_8_STR, 13),
            23514624000
        );
    }
    #[test]
    fn problem_nine() {
        assert_eq!(problem_nine::pythagorean_triplet(1000), vec![(200, 375)]);
    }
    #[test]
    fn problem_ten() {
        assert_eq!(problem_ten::sum_of_primes(2_000_000), 142913828922);
    }
}
