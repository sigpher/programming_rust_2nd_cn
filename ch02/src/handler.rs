use serde::Deserialize;

pub mod get_index;
pub mod post_gcd;
pub use get_index::get_index;
pub use post_gcd::post_gcd;

#[derive(Debug, Deserialize)]
pub struct GcdParameters {
    pub n: u64,
    pub m: u64,
}


use std::env;
use std::str::FromStr;

pub fn gcd(mut m: u64, mut n: u64) -> u64 {
    while n != 0 {
        let t = m % n;
        m = n;
        n = t;
    }
    m
}

pub fn gcd_commandline() {
    let numbers: Vec<u64> = env::args()
        .skip(1)
        .map(|arg| u64::from_str(&arg).expect("error parsing argument"))
        .collect();
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 19);
}