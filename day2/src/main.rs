use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let passwords = fs::read_to_string(filename).expect("couldn't read input file");

    let valid_sled_rental_count = passwords
        .lines()
        .filter(|line| is_valid_sled_rental_line(line))
        .count();

    println!(
        "There are {} valid passwords by sled rental rules",
        valid_sled_rental_count
    );

    let valid_tcas_count = passwords
        .lines()
        .filter(|line| is_valid_tcas_line(line))
        .count();

    println!(
        "There are {} valid passwords by TCAS rules",
        valid_tcas_count
    );
}

// 2-6 w: wkwwwfwwpvw
// 14-15 v: hvhvlhvvvwxvdvscdpvg
// 2-3 b: bkkb
// 3-4 v: wgvvzvhcvlrvjvvmv
// 11-12 c: cccccccccqcccckcc
// 2-5 z: zkzpzlzzzzz
// 7-13 x: xxxxxxxxxxxxxxx
// 7-10 f: zffxbwffqfxfffxf
// 16-17 h: hvhhhhhhhhhhhhhrt
fn is_valid_sled_rental_line(line: &str) -> bool {
    let re = Regex::new(r"^(\d+?)-(\d+?) (.): (.+)$").unwrap();
    let captures = re.captures(line).expect("to work");

    let low_bound: usize = captures
        .get(1)
        .map(|m| m.as_str().parse().unwrap())
        .unwrap();
    let high_bound: usize = captures
        .get(2)
        .map(|m| m.as_str().parse().unwrap())
        .unwrap();
    let letter: &str = captures.get(3).map(|m| m.as_str()).unwrap();
    let password = captures.get(4).map(|m| m.as_str()).unwrap();

    let count = password.matches(letter).count();

    low_bound <= count && count <= high_bound
}

// This just works on bytes as none of the input is utf8
fn is_valid_tcas_line(line: &str) -> bool {
    let re = Regex::new(r"^(\d+?)-(\d+?) (.): (.+)$").unwrap();
    let captures = re.captures(line).expect("to work");

    let first_position: usize = captures
        .get(1)
        .map(|m| m.as_str().parse().unwrap())
        .unwrap();
    let second_position: usize = captures
        .get(2)
        .map(|m| m.as_str().parse().unwrap())
        .unwrap();
    let letter: u8 = captures.get(3).map(|m| m.as_str().as_bytes()[0]).unwrap();
    let password = captures.get(4).map(|m| m.as_str().as_bytes()).unwrap();

    let first_position_valid = password[first_position - 1] == letter;
    let second_position_valid = password[second_position - 1] == letter;

    first_position_valid ^ second_position_valid
}
