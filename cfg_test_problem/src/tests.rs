//#![cfg(test)]
// Depending if ↑ this is commented out or not,
// this module is considered part of the main code vs. test suite by cargo tarpaulin.

use super::*;

pub fn test_helper() {
    println!("this shouldn't be marked as un-covered as it's in a #[cfg(test)] module!");
}

#[test]
fn test_func() {
    func();
}
