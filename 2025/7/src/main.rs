use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input file");
    let mut lines = contents.lines();

    let mut beams: Vec<i64> = Vec::new();
    for character in lines.by_ref().take(1).last().unwrap().chars() {
        match character {
            '.' => beams.push(0),
            'S' => beams.push(1),
            _ => panic!("Unexpected character in input"),
        }
    }

    let mut splits = 0;
    for line in lines.by_ref() {
        let mut last_flipped: Option<usize> = None;
        for (index, character) in line.chars().enumerate() {
            match character {
                '.' => continue,
                '^' => {
                    if beams[index] == 0
                        || last_flipped.is_some_and(|last_flipped| last_flipped == index)
                    {
                        continue;
                    }

                    splits += 1;

                    if index > 0 {
                        beams[index - 1] += beams[index];
                    }

                    if index < beams.len() - 1 {
                        if beams[index + 1] == 0 {
                            last_flipped = Some(index + 1);
                        }

                        beams[index + 1] += beams[index];
                    }

                    beams[index] = 0;
                }
                _ => panic!("Unexpected character in input"),
            }
        }
    }

    println!("Splits: {} (part one)", splits);
    println!("Timelines: {} (part two)", beams.iter().sum::<i64>());
}
