pub struct Prime {
    known_primes: Vec<u128>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PrimeFactor {
    pub base: u128,
    pub exponent: u128,
}

impl Prime {
    pub fn new() -> Prime {
        return Prime {
            known_primes: vec![2, 3, 5, 7, 11, 13, 17, 19, 21, 23],
        };
    }

    pub fn prime_factors(&mut self, number: u128) -> Vec<PrimeFactor> {
        return self.prime_factors_internal(number);
    }

    fn prime_factors_internal(&mut self, number: u128) -> Vec<PrimeFactor> {
        self.sieve_of_eratosthenes(number);

        if self.known_primes.binary_search(&number).is_ok() {
            return vec![PrimeFactor {
                base: number,
                exponent: 1u128,
            }];
        }

        let mut factors = Vec::new();
        let mut next_number = number;
        while next_number != 1 {
            for prime in &self.known_primes {
                if next_number == *prime {
                    factors.push(prime);
                    next_number = 1;
                    break;
                }

                if next_number % prime == 0 {
                    factors.push(prime);
                    next_number = next_number / prime;
                    break;
                }
            }
        }

        return Prime::extract_prime_factors(&factors);
    }

    fn extract_prime_factors(factors: &Vec<&u128>) -> Vec<PrimeFactor> {
        if factors.len() < 1 {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut current_base = factors[0];
        let mut current_exponent = 1u128;
        for factor in &factors[1..] {
            if *factor == current_base {
                current_exponent += 1;
            } else {
                result.push(PrimeFactor {
                    base: *current_base,
                    exponent: current_exponent,
                });

                current_base = *factor;
                current_exponent = 1;
            }
        }

        result.push(PrimeFactor {
            base: *current_base,
            exponent: current_exponent,
        });

        return result;
    }

    fn sieve_of_eratosthenes(&mut self, number: u128) {
        if number > usize::MAX as u128 {
            panic!("input value is too large")
        }

        if self.known_primes.len() > 0 && self.known_primes[self.known_primes.len() - 1] >= number {
            return;
        }

        self.known_primes.clear();
        let mut sieve = Vec::with_capacity(number as usize);
        for _ in 0..number + 1 {
            sieve.push(true);
        }

        let last_index = number as usize;
        for i in 2usize..last_index + 1 {
            if sieve[i] == false {
                continue;
            }

            self.known_primes.push(i as u128);
            let mut j = i * i;
            while j <= last_index {
                sieve[j] = false;
                j += i;
            }
        }
    }

    fn calculate_primes_until(&mut self, number: u128) {
        let start = if self.known_primes.len() > 0 {
            self.known_primes[self.known_primes.len() - 1] + 1
        } else {
            2
        };
        for x in start..number + 1 {
            if self.known_primes.iter().all(|prime| x % prime != 0) {
                self.known_primes.push(x);
            }
        }
    }

    fn is_prime_internal(number: u128) -> bool {
        let upper_bound = Prime::greatest_possible_prime(number);
        for divisor in 2..upper_bound + 1 {
            if number % divisor == 0 {
                return false;
            }
        }

        return true;
    }

    fn greatest_possible_prime(number: u128) -> u128 {
        (number as f64).sqrt().floor() as u128
    }
}

#[cfg(test)]
mod tests {
    use crate::day8::prime::{Prime, PrimeFactor};

    #[test]
    fn test_prime_factors() {
        let mut prime = Prime::new();

        assert_eq!(
            prime.prime_factors(2),
            vec![PrimeFactor {
                base: 2,
                exponent: 1,
            }]
        );
        assert_eq!(
            prime.prime_factors(3),
            vec![PrimeFactor {
                base: 3,
                exponent: 1,
            }]
        );
        assert_eq!(
            prime.prime_factors(4),
            vec![PrimeFactor {
                base: 2,
                exponent: 2,
            }]
        );
        assert_eq!(
            prime.prime_factors(6),
            vec![
                PrimeFactor {
                    base: 2,
                    exponent: 1,
                },
                PrimeFactor {
                    base: 3,
                    exponent: 1,
                },
            ]
        );
        assert_eq!(prime.prime_factors(3528),
                   vec![
                       PrimeFactor {
                           base: 2,
                           exponent: 3,
                       },
                       PrimeFactor {
                           base: 3,
                           exponent: 2,
                       },
                       PrimeFactor {
                           base: 7,
                           exponent: 2,
                       },
                   ]);
        assert_eq!(prime.prime_factors(8011),
                   vec![
                       PrimeFactor {
                           base: 8011,
                           exponent: 1,
                       }
                   ]);
    }
}
