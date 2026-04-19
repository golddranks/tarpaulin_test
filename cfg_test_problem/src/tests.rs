//#![cfg(test)]
// Depending if ↑ this is commented out or not,
// this module is considered part of the main code vs. test suite by cargo tarpaulin.

use super::*;

struct TestHelper;

fn test_helper() {
    println!("from test_helper");
}

#[test]
fn test_func() {
    func();
}
