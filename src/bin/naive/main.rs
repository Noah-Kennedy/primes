use std::env::args;
use primes::naive::naive_vec;

fn main() {
    let args: Vec<String> = args().collect();
    let limit = args[1].parse().unwrap();
    let primes = naive_vec(limit);
    println!("{:?}", primes);
}