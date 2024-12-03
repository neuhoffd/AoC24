//use core::fmt;
use std::{fmt::Binary, fs, string, vec};

fn main() {
    assert_eq!(play(".\\test0.txt", true), 11);
    assert_eq!(play(".\\input.txt", true), 1970720);

    assert_eq!(play(".\\test0.txt", false), 11);
    assert_eq!(play(".\\input.txt", false), 1970720);
}

fn play(path: &str, part1: bool) -> i64 {
    let mut parsed: (Vec<i64>, Vec<i64>) = parse(
        path,
        part1,
    );
    parsed.0.sort();
    parsed.1.sort();
    let mut ans: i64 = 0;
    for i in 0..parsed.0.len() {
        ans += (parsed.0[i] - parsed.1[i]).abs()
    }
    println!("Part {}: \nResult: {}",
        if part1 { 1 } else { 2 },
        ans,);
    ans
}

fn parse(path: &str, _part1: bool) -> (Vec<i64>, Vec<i64>) {
    let input = &fs::read_to_string(path)
            .map(string::String::into_boxed_str)
            .unwrap();
    input.lines().map(|line| -> (i64, i64) {
        let mut splits = line.split_whitespace();
        (splits.next().unwrap().parse::<i64>().unwrap(), splits.last().unwrap().parse::<i64>().unwrap())
    }).unzip()
}
