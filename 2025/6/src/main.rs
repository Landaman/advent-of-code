use std::fs;

enum Operator {
    Multiply,
    Add,
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input file");
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

    println!("Result: {}", values.iter().sum::<i64>());
}
