#[macro_use]
#[cfg(test)]
extern crate quickcheck_macros;

pub mod eratosthenes;
pub mod atkin;
pub mod naive;

#[cfg(test)]
pub mod properties;

fn main() {
    println!("Hello, world!");
}
