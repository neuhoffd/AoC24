//use core::fmt;
use std::{collections::HashMap, fs, string};

fn main() {
    assert_eq!(play(".\\test0.txt", true), 2);
    assert_eq!(play(".\\input.txt", true), 306);

    assert_eq!(play(".\\test0.txt", false), 4);
    assert_eq!(play(".\\input.txt", false), 17191599);
}

fn is_safe(report: &Vec<i64>) -> i64 {
    let increments = increments(report);
    if increments
        .iter()
        .all(|incr| incr.signum() == increments[0].signum() && incr.abs() <= 3 && incr.abs() >= 1)
    {
        1
    } else {
        0
    }
}

fn increments(report: &Vec<i64>) -> Vec<i64> {
    report[0..report.len() - 1]
        .iter()
        .zip(&report[1..report.len()])
        .map(|(a, b)| -> i64 { b - a })
        .collect()
}

fn part1(parsed: Vec<Vec<i64>>) -> i64 {
    parsed.into_iter().map(|report| is_safe(&report)).sum()
}

fn part2(parsed: Vec<Vec<i64>>) -> i64 {
    parsed
        .into_iter()
        .map(|report| -> i64 {
            if is_safe(&report) > 0 {
                println!("Report {:?} is safe", report);
                return 1;
            }
            for remove in 0..report.len() {
                let mut without_remove = report.clone();
                without_remove.remove(remove);
                println!("Remove {} \n {:?}", remove, without_remove);
                if is_safe(&without_remove) > 0 {
                    println!(
                        "Report {:?} is safe due to removal of level # {}",
                        report, remove
                    );
                    return 1;
                }
            }
            println!("Report {:?} is unsafe", report);
            0
        })
        .sum()
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
