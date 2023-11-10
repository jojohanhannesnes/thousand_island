mod utils;
use std::collections::{BTreeMap, HashMap};

use utils::list_of_prime_factor;

fn smallest_multiple() -> i32 {
    1
}

fn main() {
    let mut db = BTreeMap::new();
    for i in 1..20 {
        let prime_factors = list_of_prime_factor(i);
        db.insert(i, prime_factors);
    }
    let mut max_occurrences: BTreeMap<i64, usize> = BTreeMap::new();

    for values in db.values() {
        for &num in values {
            let count = max_occurrences.entry(num).or_insert(0);
            *count = (*count).max(values.iter().filter(|&&x| x == num).count());
        }
    }

    let result = max_occurrences.iter().fold(1, |acc, (&num, &count)| {
        acc * (num as u32).pow(count as u32)
    });
    println!("{}", result);
}