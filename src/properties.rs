use quickcheck::TestResult;

pub fn div_by(factor: u64, prime_generator: fn(u64) -> Vec<u64>, limit: u64) -> TestResult {
    if factor < 2 || limit < 2 {
        TestResult::discard()
    } else {
        let primes = prime_generator(limit);
        for prime in primes {
            if prime % factor == 0 && prime != factor {
                return TestResult::failed();
            }
        }

        TestResult::passed()
    }
}