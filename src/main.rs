#![feature(let_chains)]
#![allow(dead_code)]
#![feature(variant_count)]
#![feature(iter_map_windows)]
#![feature(iter_intersperse)]
#![feature(if_let_guard)]
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod template;

use std::{env, fs};

type Solution = fn(&str) -> String;

#[derive(Default)]
pub struct Solutions(&'static [(Solution, &'static str)]);

impl Solutions {
    const SOLUTIONS: Solutions = Solutions(&[
        (day01::p1, "inputs/1.txt"),
        (day01::p2, "inputs/1.txt"),
        (day02::p1, "inputs/2.txt"),
        (day02::p2, "inputs/2.txt"),
        (day03::p1, "inputs/3.txt"),
        (day03::p2, "inputs/3.txt"),
        (day04::p1, "inputs/4.txt"),
        (day04::p2, "inputs/4.txt"),
        (day05::p1, "inputs/5.txt"),
        (day05::p2, "inputs/5.txt"),
        (day06::p1, "inputs/6.txt"),
        (day06::p2, "inputs/6.txt"),
        (day07::p1, "inputs/7.txt"),
        (day07::p2, "inputs/7.txt"),
        (day08::p1, "inputs/8.txt"),
        (day08::p2, "inputs/8.txt"),
        (day09::p1, "inputs/9.txt"),
        (day09::p2, "inputs/9.txt"),
        (day10::p1, "inputs/10.txt"),
        (day10::p2, "inputs/10.txt"),
        (day11::p1, "inputs/11.txt"),
        (day11::p2, "inputs/11.txt"),
        (day12::p1, "inputs/12.txt"),
        (day12::p2, "inputs/12.txt"),
        (day13::p1, "inputs/13.txt"),
        (day13::p2, "inputs/13.txt"),
        (day14::p1, "inputs/14.txt"),
        (day14::p2, "inputs/14.txt"),
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
    ]);

    pub fn run_all() {
        for (i, (f, path)) in Solutions::SOLUTIONS.0.iter().enumerate() {
            let c = if i % 2 == 0 { "a" } else { "b" };
            println!("{}{}: {}", (i / 2) + 1, c, f(&Solutions::get_input(path)));
        }
    }
    pub fn get_input(path: &str) -> String {
        fs::read_to_string(path).expect("failed to read input file")
    }
}

pub fn main() {
    if let Some(e) = env::args().nth(1)
        && e.len() > 1
    {
        let (num, char): (usize, char) = (
            e[0..(e.len() - 1)].parse().unwrap(),
            e.chars().nth(e.len() - 1).unwrap(),
        );
        let o = match char {
            'a' => 0,
            'b' => 1,
            _ => panic!("Please specify part 1 or 2 with arg n{{a|b}}"),
        };
        let (f, path) = Solutions::SOLUTIONS.0[(num - 1) * 2 + o];
        println!("{}", f(&fs::read_to_string(path).unwrap()));
    } else {
        Solutions::run_all();
    }
}
