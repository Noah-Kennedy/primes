#[cfg(test)]
mod tests;

pub fn naive_vec(limit: u64) -> Vec<u64> {
    let mut results = Vec::new();

    for i in 2..=limit {
        if is_prime_naive(i) {
            results.push(i);
        }
    }

    results
}

#[inline]
pub fn is_prime_naive(num: u64) -> bool {
    match num {
        0 | 1 => panic!("Cannot check if 0 or 1 is prime!"),
        2 => true,
        n => {
            let mut is_prime = true;

            for i in 2..n {
                if n % i == 0 {
                    is_prime = false;
                    break
                }
            }

            is_prime
        }
    }
}