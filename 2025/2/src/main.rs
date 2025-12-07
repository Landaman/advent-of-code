use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    let mut result: i64 = 0;

    input.split(",").for_each(|range| {
        if range.is_empty() {
            return;
        }

        let string_split = range
            .split_once("-")
            .expect("Found a range containing no '-'");
        let lower = string_split.0.parse::<i64>().unwrap();
        let upper = string_split.1.trim_end().parse::<i64>().unwrap();

        let mut number = lower;
        while number <= upper {
            let number_string = number.to_string();
            if number_string.len() % 2 != 0 {
                // skip odd
                number += 1;
                continue;
            }
            let number_parts = number_string.split_at(number_string.len() / 2);

            if number_parts.0 == number_parts.1 {
                result += number;
            }

            number += 1;
        }
    });

    println!("Sum of invalid IDs: {}", result);
}
