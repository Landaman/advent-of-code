use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input file");

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

    println!("Found {} spots", result);
}
