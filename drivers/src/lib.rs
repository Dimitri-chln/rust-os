#![no_std]
// Tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(utils::test::runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

extern crate alloc;

pub mod display;
pub mod fs;
