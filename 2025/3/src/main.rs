use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    let mut result_one = 0;
    let mut result_two: i64 = 0;
    input.split("\n").for_each(|line| {
        if line.is_empty() {
            return;
        }

        if line.len() <= 1 {
            let value = line.to_string().parse::<i32>().unwrap_or(0);
            result_one += value;
            result_two += value as i64;
            return;
        }

        let mut highest_one = 0;
        let mut highest_two = 0;
        let mut highest_twelve = [0; 12];
        let mut dirty_index = 0;

        for (index, character) in line.chars().enumerate() {
            let number = character
                .to_string()
                .parse::<i32>()
                .expect("Failed to parse number");

            if number > highest_one && index != line.len() - 1 {
                // Last one can't be 1st highest
                highest_one = number;
                highest_two = 0;
            } else if number > highest_two {
                highest_two = number;
            }

            for (highest_index, _) in highest_twelve.iter().enumerate() {
                if highest_index == dirty_index {
                    highest_twelve[highest_index] = number;
                    dirty_index += 1;
                    break;
                }

                // How many elements left must be >= how many elements we need
                if number > highest_twelve[highest_index]
                    && line.len() - index >= highest_twelve.len() - highest_index
                {
                    highest_twelve[highest_index] = number;
                    dirty_index = highest_index + 1;
                    break;
                }
            }
        }

        result_one += highest_one * 10 + highest_two;
        result_two +=
            highest_twelve
                .iter()
                .take(dirty_index)
                .enumerate()
                .fold(0, |acc, (index, element)| {
                    acc + element.to_owned() as i64
                        * (10_i64).pow((highest_twelve.len() - index - 1) as u32)
                })
    });

    println!(
        "Highest possible count: {} (part one), {} (part two)",
        result_one, result_two
    );
}
