// -- Least common multiple and Greatest common divisor --
// This example is used to demonstrate `bintest_helper`, a helper for the `assert_cmd`
// crate on crates.io.
// This incidentally was the reason I wrote this: to test this binary (see `main()`)
//
// Original author and language: Tsoding, Haskell
//
// DISCLAIMER: this is not good rust code, it is just Tsoding's
// HaskellRank code (https://www.youtube.com/watch?v=40kpc90ZzDg)
// squashed into rust.

use std::{cmp::min, io, mem::swap};

#[cfg(test)]
mod tests {
    use super::*;

    // Define a test function, which uses bintest_helper.
    #[bintest_helper::bintest_helper]
    #[test]
    fn main_test() { // After this line compilation of your entire Cargo.toml folder starts immediately.
        // Now you can use the new binary...
        use assert_cmd::Command;

        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        // ... and test it.
        cmd.write_stdin("2 3\n2 4\n16 32 96").assert().stdout("3\n");
    }
    #[test]
    fn gcd_test() {
        assert_eq!(gcd(2, 6), 2);
        assert_eq!(gcd(6, 12), 6);
        assert_eq!(gcd(15, 20), 5);
    }
    #[test]
    fn lcm_test() {
        assert_eq!(lcm(2, 6), 6);
        assert_eq!(lcm(6, 12), 12);
        assert_eq!(lcm(15, 20), 60);
    }
}

// Everything after this is just the solution to the problem on HackerRank

fn main() {
    if let [_n, _m] = read_int_list()[..] {
        let a = read_int_list();
        let b = read_int_list();
        println!("{}", solve(a, b));
    }
}

pub fn solve(a: Vec<usize>, b: Vec<usize>) -> usize {
    let a_lcm = a.into_iter().reduce(lcm).unwrap();
    let b_gcd = b.into_iter().reduce(gcd).unwrap();
    (1..)
        .map(|x| x * a_lcm)
        .take_while(|x| x <= &b_gcd)
        .filter(|x| b_gcd % x == 0)
        .count()
}

fn read_int_list() -> Vec<usize> {
    let stdin = io::stdin();
    let mut line = String::new();
    let _ = stdin.read_line(&mut line);
    line.trim()
        .split(" ")
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect()
}

// How to implement gcd and lcm:
// https://en.wikipedia.org/wiki/Binary_GCD_algorithm#Implementation
// My implementation is much simpler though, through some pattern matching
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        swap(&mut a, &mut b);
    }
    match (a, b) {
        (a, 0) => a,
        (0, b) => b,
        _ => match (a % 2, b % 2) {
            (0, 0) => 2 * gcd(a / 2, b / 2),
            (0, _) => gcd(a / 2, b),
            (_, 0) => gcd(a, b / 2),
            _ => gcd(a - b, min(a, b)),
        },
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}
