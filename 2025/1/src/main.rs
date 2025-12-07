use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input file");
    let mut dial = 50;
    let mut zero_count_one = 0;
    let mut zero_count_two = 0;

    contents.split('\n').for_each(|line| {
        if line.is_empty() {
            return;
        }

        let base_amount = line
            .chars()
            .skip_while(|character| character.is_alphabetic())
            .collect::<String>()
            .parse::<i32>()
            .expect("Invalid input, failed to parse number");
        let amount = base_amount % 100; // mod 100 to keep within dial range
        let spins = base_amount / 100; // floor division number of full spins

        let direction = line
            .chars()
            .take_while(|character| character.is_alphabetic())
            .collect::<String>();

        if direction == "R" {
            dial += amount;
            if dial >= 100 {
                dial -= 100;

                zero_count_two += 1;
            }
        } else if direction == "L" {
            let started_on_zero = dial == 0;

            dial -= amount;
            if dial < 0 {
                dial += 100;

                if !started_on_zero {
                    zero_count_two += 1;
                }
            } else if dial == 0 && !started_on_zero {
                zero_count_two += 1;
            }
        } else {
            panic!(
                "Invalid input, expected 'L' or 'R' direction, got '{}'",
                direction
            );
        }

        zero_count_two += spins;

        if dial == 0 {
            zero_count_one += 1;
        }
    });

    println!(
        "Zero count: {} (part one), {} (part two)",
        zero_count_one, zero_count_two
    );
}
