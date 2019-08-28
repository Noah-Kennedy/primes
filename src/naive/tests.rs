use super::*;
use crate::properties::div_by;
use quickcheck::TestResult;

#[quickcheck]
fn naive_factor_test(factor: u64, limit: u64) -> TestResult {
    div_by(factor, naive_vec as fn(u64) -> Vec<u64>, limit)
}