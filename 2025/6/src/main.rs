use std::fs;

enum Operator {
    Multiply,
    Add,
}

fn part_one(contents: &str) {
    let mut lines_iter = contents.lines();

    let mut operators: Vec<Operator> = Vec::new();
    let mut values: Vec<i64> = Vec::new();

    for string in lines_iter.by_ref().rev().take(1).last().unwrap().split(' ') {
        // May have multiple spaces chained
        if string.is_empty() {
            continue;
        }

        if string == "*" {
            operators.push(Operator::Multiply);
            values.push(1);
        } else if string == "+" {
            operators.push(Operator::Add);
            values.push(0);
        } else {
            panic!("Unknown operator: {}", string);
        }
    }

    for line in lines_iter.by_ref() {
        let mut index = 0;
        for sring in line.split(' ') {
            // May have multiple spaces chained
            if sring.is_empty() {
                continue;
            }

            let value: i64 = sring.parse().expect("Failed to parse number");
            values[index] = match operators[index] {
                Operator::Multiply => values[index] * value,
                Operator::Add => values[index] + value,
            };

            index += 1;
        }
    }

    println!("Result One: {}", values.iter().sum::<i64>());
}

fn part_two(contents: &str) {
    let mut lines_iter = contents.lines();
    let mut values: Vec<i64> = Vec::new();

    for line in lines_iter.by_ref().take(contents.lines().count() - 1) {
        for (index, character) in line.chars().enumerate() {
            // Should only happen on 1st line
            if values.len() <= index {
                values.push(0);
            }

            if character.is_whitespace() {
                continue;
            }

            let digit = character.to_digit(10).expect("Failed to parse digit") as i64;
            values[index] *= 10; // Shift left. If this is the 1st digit, this does nothing
            values[index] += digit;
        }
    }

    let mut result = 0;
    let mut column_result = 0;
    let mut operator: Option<Operator> = None;
    let last_line_characters = lines_iter.by_ref().last().unwrap();
    for (index, character) in last_line_characters.chars().enumerate() {
        if character == '*' {
            result += column_result;
            column_result = 1;
            operator = Some(Operator::Multiply);
        } else if character == '+' {
            result += column_result;
            column_result = 0;
            operator = Some(Operator::Add);
        } else if !character.is_whitespace() {
            panic!("Unknown operator: {}", character);
        }

        if values[index] == 0 {
            continue;
        }

        match operator.as_ref().unwrap() {
            Operator::Multiply => {
                column_result *= values[index];
            }
            Operator::Add => {
                column_result += values[index];
            }
        }
    }

    // Add anything left, since the last line won't have anything after the last op
    for value in values.iter().skip(last_line_characters.len()) {
        column_result += *value;
    }
    result += column_result;

    println!("Result Two: {}", result);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input file");
    part_one(&contents);
    part_two(&contents);
}
