#![feature(let_chains)]
#![allow(dead_code)]
#![feature(variant_count)]
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use std::fs;

type Solution = fn(&str) -> String;

#[derive(Default)]
pub struct Solutions(&'static [(Solution, &'static str)]);

impl Solutions {
    pub fn new() -> Solutions {
        Solutions(&[
            (day1::p1, "inputs/1.txt"),
            (day1::p2, "inputs/1.txt"),
            (day2::p1, "inputs/2.txt"),
            (day2::p2, "inputs/2.txt"),
            (day3::p1, "inputs/3.txt"),
            (day3::p2, "inputs/3.txt"),
            (day4::p1, "inputs/4.txt"),
            (day4::p2, "inputs/4.txt"),
            (day5::p1, "inputs/5.txt"),
            (day5::p2, "inputs/5.txt"),
            (day6::p1, "inputs/6.txt"),
            (day6::p2, "inputs/6.txt"),
            (day7::p1, "inputs/7.txt"),
            (day7::p2, "inputs/7.txt"),
            // (day8::p1, "inputs/8.txt"),
            // (day8::p2, "inputs/8.txt"),
            // (day9::p1, "inputs/9.txt"),
            // (day9::p2, "inputs/9.txt"),
            // (day10::p1, "inputs/10.txt"),
            // (day10::p2, "inputs/10.txt"),
            // (day11::p1, "inputs/11.txt"),
            // (day11::p2, "inputs/11.txt"),
            // (day12::p1, "inputs/12.txt"),
            // (day12::p2, "inputs/12.txt"),
            // (day13::p1, "inputs/13.txt"),
            // (day13::p2, "inputs/13.txt"),
            // (day14::p1, "inputs/14.txt"),
            // (day14::p2, "inputs/14.txt"),
            // (day15::p1, "inputs/15.txt"),
            // (day15::p2, "inputs/15.txt"),
            // (day16::p1, "inputs/16.txt"),
            // (day16::p2, "inputs/16.txt"),
            // (day17::p1, "inputs/17.txt"),
            // (day17::p2, "inputs/17.txt"),
            // (day18::p1, "inputs/18.txt"),
            // (day18::p2, "inputs/18.txt"),
            // (day19::p1, "inputs/19.txt"),
            // (day19::p2, "inputs/19.txt"),
            // (day20::p1, "inputs/20.txt"),
            // (day20::p2, "inputs/20.txt"),
            // (day21::p1, "inputs/21.txt"),
            // (day21::p2, "inputs/21.txt"),
            // (day22::p1, "inputs/22.txt"),
            // (day22::p2, "inputs/22.txt"),
            // (day23::p1, "inputs/23.txt"),
            // (day23::p2, "inputs/23.txt"),
            // (day24::p1, "inputs/24.txt"),
            // (day24::p2, "inputs/24.txt"),
            // (day25::p1, "inputs/25.txt"),
            // (day25::p2, "inputs/25.txt"),
        ])
    }
    pub fn run_all(&self) {
        for (i, (f, path)) in self.0.iter().enumerate() {
            let c = if i % 2 == 0 { "a" } else { "b" };
            println!("{}{}: {}", (i / 2) + 1, c, f(&Solutions::get_input(path)));
        }
    }
    pub fn get_input(path: &str) -> String {
        fs::read_to_string(path).expect("failed to read input file")
    }
}

pub fn main() {
    Solutions::new().run_all();
}
