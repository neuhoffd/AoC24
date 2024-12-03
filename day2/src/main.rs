//use core::fmt;
use std::{collections::HashMap, fs, string};

fn main() {
    assert_eq!(play(".\\test0.txt", true), 2);
    assert_eq!(play(".\\input.txt", true), 1970720);

    assert_eq!(play(".\\test0.txt", false), 31);
    assert_eq!(play(".\\input.txt", false), 17191599);
}

fn part1(parsed: Vec<Vec<i64>>) -> i64 {
    let mut ans: i64 = 0;

    parsed
        .into_iter()
        .map(|line| -> i64 {
            let increments: Vec<i64> = line[0..line.len() - 1]
                .iter()
                .zip(&line[1..line.len()])
                .map(|(a, b)| -> i64 { b - a })
                .collect();
            if increments
                .iter()
                .map(|incr| -> (i64, i64) { (*incr, incr.signum()) })
                .all(|(incr, sign)| sign == increments[0].signum() && incr.abs() <= 3)
            {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part2(parsed: Vec<Vec<i64>>) -> i64 {
    let mut ans: i64 = 0;

    ans
}

fn play(path: &str, ispart1: bool) -> i64 {
    let mut parsed: Vec<Vec<i64>> = parse(path, ispart1);
    //println!(":?#", parsed);
    let mut ans = 0i64;

    if ispart1 {
        ans = part1(parsed);
    } else {
        ans = part2(parsed);
    }
    println!("Part {}: \nResult: {}", if ispart1 { 1 } else { 2 }, ans,);
    ans
}

fn parse(path: &str, _part1: bool) -> (Vec<Vec<i64>>) {
    let input = &fs::read_to_string(path)
        .map(string::String::into_boxed_str)
        .unwrap();
    input
        .lines()
        .map(|line| -> Vec<i64> {
            line.split_whitespace()
                .map(|i| -> i64 { i.parse::<i64>().unwrap() })
                .collect()
        })
        .collect()
}
