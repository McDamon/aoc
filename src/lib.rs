#![feature(let_chains)]
#![feature(test)]

#[macro_use]
extern crate lazy_static;

#[path = "./utils/utils.rs"]
#[allow(dead_code)]
mod utils;
#[allow(dead_code)]
mod aoc2024;