//use core::fmt;
use std::{collections::HashMap, fs, string};

fn main() {
    assert_eq!(play(".\\test0.txt", true), 11);
    assert_eq!(play(".\\input.txt", true), 1970720);

    assert_eq!(play(".\\test0.txt", false), 31);
    assert_eq!(play(".\\input.txt", false), 17191599);
}

fn part1(parsed: (Vec<i64>, Vec<i64>)) -> i64 {
    let mut ans: i64 = 0;

    ans
}

fn part2(parsed: (Vec<i64>, Vec<i64>)) -> i64 {
    let mut ans: i64 = 0;

    ans
}

fn play(path: &str, ispart1: bool) -> i64 {
    let mut parsed: (Vec<i64>, Vec<i64>) = parse(path, ispart1);
    let mut ans = 0i64;

    if ispart1 {
        ans = part1(parsed);
    } else {
        ans = part2(parsed);
    }
    println!("Part {}: \nResult: {}", if ispart1 { 1 } else { 2 }, ans,);
    ans
}

fn parse(path: &str, _part1: bool) -> (Vec<i64>, Vec<i64>) {
    let input = &fs::read_to_string(path)
        .map(string::String::into_boxed_str)
        .unwrap();
    input.lines().map(|line| -> (i64, i64) { (0, 0) }).unzip()
}
