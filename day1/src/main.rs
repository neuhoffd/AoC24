use core::fmt;
use std::{fs, string, vec};

fn main() {
    assert_eq!(play(".\\test0.txt", true), 62);
    assert_eq!(play(".\\input.txt", true), 95356);

    assert_eq!(play(".\\test0.txt", true), 952408144115);
    assert_eq!(play(".\\input.txt", true), 92291468914147);
}


fn parse(input: &str, part1: bool) -> usize {
    
}
