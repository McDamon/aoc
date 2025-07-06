#![feature(test)]

#[macro_use]
extern crate lazy_static;

pub mod aoc2019;
pub mod aoc2024;

#[path = "./utils/utils.rs"]
pub mod utils;

#[path = "./utils/intcode.rs"]
pub mod intcode;
