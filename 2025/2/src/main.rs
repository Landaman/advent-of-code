use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    let mut result_one: i64 = 0;
    let mut result_two: i64 = 0;
    let regex = pcre2::bytes::Regex::new(r"^(.+)(\g1+)$").unwrap();

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

            let number_parts = number_string.split_at(number_string.len() / 2);
            if number_parts.0 == number_parts.1 {
                result_one += number;
            }

            if regex.is_match(number_string.as_bytes()).unwrap() {
                result_two += number;
            }

            number += 1;
        }
    });

    println!(
        "Sum of invalid IDs: {} (part one), {} (part two)",
        result_one, result_two
    );
}
