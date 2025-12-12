use std::fs;

fn part1(contents: &str) {
    let mut result = 0;
    let mut last_line: Option<Vec<Option<i32>>> = Option::None;
    contents.split("\n").for_each(|line| {
        let mut this_line: Vec<Option<i32>> = Vec::new();

        if line.is_empty() {
            return;
        }

        line.chars().enumerate().for_each(|(index, character)| {
            let this_value: Option<i32>;
            if character == '@' {
                let mut this_value_num = 0;

                if index != 0
                    && let Some(Some(this_last)) = this_line.get_mut(index - 1)
                {
                    *this_last += 1;
                    this_value_num += 1;
                }

                if let Some(last_line) = last_line.as_mut() {
                    if let Some(Some(last_above)) = last_line.get_mut(index) {
                        *last_above += 1;
                        this_value_num += 1;
                    }

                    if index != 0
                        && let Some(Some(last_left)) = last_line.get_mut(index - 1)
                    {
                        *last_left += 1;
                        this_value_num += 1;
                    }

                    if index != last_line.len() - 1
                        && let Some(Some(last_right)) = last_line.get_mut(index + 1)
                    {
                        *last_right += 1;
                        this_value_num += 1;
                    }
                }
                this_value = Some(this_value_num);
            } else if character == '.' {
                this_value = Option::None;
            } else {
                panic!("Unexpected character in input");
            }

            this_line.push(this_value);

            if index != 0
                && let Some(last_line) = last_line.as_ref()
                && let Some(Some(last_left)) = last_line.get(index - 1)
                && *last_left < 4
            {
                result += 1;
            }

            if let Some(last_line) = last_line.as_ref()
                && index == last_line.len() - 1
                && let Some(Some(last_above)) = last_line.get(index)
                && *last_above < 4
            {
                result += 1;
            }
        });

        last_line = Some(this_line);
    });

    for value in last_line.expect("Got no lines") {
        if let Some(value) = value
            && value < 4
        {
            result += 1;
        }
    }

    println!("Found {} spots (part 1)", result);
}

fn part2(contents: &str) {
    let mut result = 0;

    let mut lines: Vec<Vec<Option<i32>>> = Vec::new();

    // Pre-compute count for each line
    for (line_index, line) in contents.lines().enumerate() {
        let mut this_line: Vec<Option<i32>> = Vec::new();
        let mut last_line: Option<&mut Vec<Option<i32>>> = match line_index {
            0 => None,
            _ => Some(lines.get_mut(line_index - 1).unwrap()),
        };

        for (character_index, character) in line.chars().enumerate() {
            let this_value: Option<i32>;
            if character == '@' {
                let mut this_value_num = 0;

                if character_index != 0
                    && let Some(Some(this_last)) = this_line.get_mut(character_index - 1)
                {
                    *this_last += 1;
                    this_value_num += 1;
                }

                if let Some(ref mut last_line) = last_line {
                    if let Some(Some(last_above)) = last_line.get_mut(character_index) {
                        *last_above += 1;
                        this_value_num += 1;
                    }

                    if character_index != 0
                        && let Some(Some(last_left)) = last_line.get_mut(character_index - 1)
                    {
                        *last_left += 1;
                        this_value_num += 1;
                    }

                    if character_index != last_line.len() - 1
                        && let Some(Some(last_right)) = last_line.get_mut(character_index + 1)
                    {
                        *last_right += 1;
                        this_value_num += 1;
                    }
                }
                this_value = Some(this_value_num);
            } else if character == '.' {
                this_value = Option::None;
            } else {
                panic!("Unexpected character in input");
            }

            this_line.push(this_value);
        }

        lines.push(this_line);
    }

    let mut row = 0;
    let mut column = 0;
    while row < lines.len() && column < lines[row].len() {
        if let Some(value) = lines[row][column]
            && value < 4
        {
            if row > 0
                && let Some(row) = lines.get_mut(row - 1)
            {
                clear_row(row, column, false);
            }

            if let Some(row) = lines.get_mut(row) {
                clear_row(row, column, true);
                result += 1;
            }
            if row < lines.len() - 1
                && let Some(row) = lines.get_mut(row + 1)
            {
                clear_row(row, column, false);
            }

            row = 0;
            column = 0;
        }

        column += 1;
        if column == lines[row].len() {
            column = 0;
            row += 1;
        }
    }

    println!("Cleared {} spots (part 2)", result);
}

fn clear_row(row: &mut [Option<i32>], column: usize, this_row_cleared: bool) {
    if column > 0
        && let Some(Some(left)) = row.get_mut(column - 1)
    {
        *left -= 1;
    }

    if let Some(this) = row.get_mut(column) {
        if !this_row_cleared && let Some(this) = this {
            *this -= 1;
        } else if this_row_cleared {
            *this = Option::None;
        }
    }

    if column < row.len() - 1
        && let Some(Some(right)) = row.get_mut(column + 1)
    {
        *right -= 1;
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input file");
    part1(&contents);
    part2(&contents);
}
