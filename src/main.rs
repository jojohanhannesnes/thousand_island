fn main() {
    summation_of_primes(2_000_000);
}

pub fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..((limit as f64).sqrt() as usize + 1) {
        if is_prime[i] {
            for j in (i * i..limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    (2..limit).filter(|&x| is_prime[x]).collect()
}

fn summation_of_primes(limit: usize) {
    let primes = sieve_of_eratosthenes(limit);
    let x = primes.iter().sum::<usize>();
    dbg!(x);
}
