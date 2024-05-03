#![feature(let_chains)]
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
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod template;
mod utils;
mod day23;

use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    day: usize,
    #[arg(short, long)]
    part: usize,
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input = args.file.unwrap_or(format!("inputs/{}.txt", args.day));
    let input = &fs::read_to_string(input).expect("Specified/default file not available");
    let input = &input.trim_end();
    let o = match (args.day, args.part) {
        (1, 1) => day01::p1(input),
        (1, 2) => day01::p2(input),
        (2, 1) => day02::p1(input),
        (2, 2) => day02::p2(input),
        (3, 1) => day03::p1(input),
        (3, 2) => day03::p2(input),
        (4, 1) => day04::p1(input),
        (4, 2) => day04::p2(input),
        (5, 1) => day05::p1(input),
        (5, 2) => day05::p2(input),
        (6, 1) => day06::p1(input),
        (6, 2) => day06::p2(input),
        (7, 1) => day07::p1(input),
        (7, 2) => day07::p2(input),
        (8, 1) => day08::p1(input),
        (8, 2) => day08::p2(input),
        (9, 1) => day09::p1(input),
        (9, 2) => day09::p2(input),
        (10, 1) => day10::p1(input),
        (10, 2) => day10::p2(input),
        (11, 1) => day11::p1(input),
        (11, 2) => day11::p2(input),
        (12, 1) => day12::p1(input),
        (12, 2) => day12::p2(input),
        (13, 1) => day13::p1(input),
        (13, 2) => day13::p2(input),
        (14, 1) => day14::p1(input),
        (14, 2) => day14::p2(input),
        (15, 1) => day15::p1(input),
        (15, 2) => day15::p2(input),
        (16, 1) => day16::p1(input),
        (16, 2) => day16::p2(input),
        (17, 1) => day17::p1(input),
        (17, 2) => day17::p2(input),
        (18, 1) => day18::p1(input),
        (18, 2) => day18::p2(input),
        (19, 1) => day19::p1(input),
        (19, 2) => day19::p2(input),
        (20, 1) => day20::p1(input),
        (20, 2) => day20::p2(input),
        (21, 1) => day21::p1(input),
        (21, 2) => day21::p2(input),
        (22, 1) => day22::p1(input),
        (22, 2) => day22::p2(input),
        (23, 1) => day23::p1(input),
        (23, 2) => day23::p2(input),
        (24, 1) => unimplemented!(),
        (24, 2) => unimplemented!(),
        (25, 1) => unimplemented!(),
        (25, 2) => unimplemented!(),
        _ => panic!("Please enter a valid puzzle input"),
    };
    println!("{}", o);
}
