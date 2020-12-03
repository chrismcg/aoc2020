use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Part 1: {}", run_part1(filename));
    println!("Part 2: {}", run_part2(filename));
}

fn run_part1(filename: &str) -> usize {
    let (area, width, height) = build_area(filename);

    tree_count(&area, 3, 1, width, height)
}

fn run_part2(filename: &str) -> usize {
    let (area, width, height) = build_area(filename);
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .iter()
        .map(|(right, down)| tree_count(&area, *right, *down, width, height))
        .product()
}

fn build_area(filename: &str) -> (Vec<u8>, usize, usize) {
    let contents = fs::read_to_string(filename).expect("input file to exist");
    let width = contents.lines().next().unwrap().len();
    let height = contents.lines().count();

    let area: Vec<u8> = contents
        .lines()
        .flat_map(|line| line.to_string().into_bytes())
        .collect();

    (area, width, height)
}

fn tree_count(
    area: &[u8],
    right_steps: usize,
    down_steps: usize,
    width: usize,
    height: usize,
) -> usize {
    calculate_indices(right_steps, down_steps, width, height)
        .iter()
        .map(|i| if area[*i] == b'#' { 1 } else { 0 })
        .sum()
}

fn calculate_indices(
    right_steps: usize,
    down_steps: usize,
    width: usize,
    height: usize,
) -> Vec<usize> {
    let mut positions: Vec<(usize, usize)> = vec![];
    let mut current_x = 0;
    let mut current_y = 0;

    while current_y < height - 1 {
        current_x += right_steps;
        current_y += down_steps;
        positions.push((current_x, current_y))
    }

    positions
        .iter()
        .map(|(x, y)| (x % width) + (y * width))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let filename = "input/example.txt";

        assert_eq!(run_part1(filename), 7)
    }

    #[test]
    fn part2_example() {
        let filename = "input/example.txt";
        assert_eq!(run_part2(filename), 336);
    }
}
