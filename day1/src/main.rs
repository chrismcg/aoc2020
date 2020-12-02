use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("couldn't read input file");

    let numbers: Vec<u32> = contents
        .lines()
        .map(|s| s.parse().expect("could parse as u32"))
        .collect();

    let (number, other_number) = two_numbers(&numbers, 2020);
    let two_product = number * other_number;
    println!(
        "numbers are {} and {}, two numbers answer is {}",
        number, other_number, two_product
    );

    let (first_number, second_number, third_number) = three_numbers(&numbers, 2020);
    let three_product = first_number * second_number * third_number;
    println!(
        "numbers are {}, {}, and {}, three numbers answer is {}",
        first_number, second_number, third_number, three_product
    );
}

fn two_numbers(numbers: &[u32], target: u32) -> (u32, u32) {
    for (i, number) in numbers.iter().enumerate() {
        for (j, other_number) in numbers.iter().enumerate() {
            if i != j && (number + other_number) == target {
                return (*number, *other_number);
            }
        }
    }

    (0, 0)
}

fn three_numbers(numbers: &[u32], target: u32) -> (u32, u32, u32) {
    for (i, first_number) in numbers.iter().enumerate() {
        for (j, second_number) in numbers.iter().enumerate() {
            for (k, third_number) in numbers.iter().enumerate() {
                if i != j && j != k && (first_number + second_number + third_number) == target {
                    return (*first_number, *second_number, *third_number);
                }
            }
        }
    }

    (0, 0, 0)
}
