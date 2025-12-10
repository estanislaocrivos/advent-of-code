macro_rules! puzzle_input {
    () => {
        include_str!("../resources/puzzle_02_input.csv")
    };
}

/// Solution to Puzzle 02 Part 01
pub fn puzzle_02_01() {
    let instructions = puzzle_input!();
    let mut solution: usize = 0;
    for range_str in instructions.split(',') {
        let range_str = range_str.trim();
        if range_str.is_empty() {
            continue;
        }
        let parts: Vec<&str> = range_str.split('-').collect();
        let start: usize = parts[0].parse().unwrap();
        let end: usize = parts[1].parse().unwrap();
        for number in start..end + 1 {
            // The idea is to try to split the number (considering it as a string) into two parts (e.g. number 123456 can be split into 123, 456)
            if number_is_splittable(number.to_string(), 2) {
                // Then check if both parts are equal. If they are, you add it to solution
                match split_number_in_n_parts(number.to_string(), 2 as usize) {
                    Ok(parts_array) => {
                        if is_invalid_id(parts_array) {
                            solution += number;
                        }
                    }
                    Err(error) => {
                        print!("{}", error);
                        continue;
                    }
                };
                continue;
            }
        }
    }
    println!("The solution for Puzzle 02 Part 01 is: {}", solution);
}

/// Solution to Puzzle 02 Part 02
pub fn puzzle_02_02() {
    let instructions = puzzle_input!();
    let mut solution: usize = 0;
    for range_str in instructions.split(',') {
        let range_str = range_str.trim();
        if range_str.is_empty() {
            continue;
        }
        let parts: Vec<&str> = range_str.split('-').collect();
        let start: usize = parts[0].parse().unwrap();
        let end: usize = parts[1].parse().unwrap();
        for number in start..end + 1 {
            // The idea is to start trying to split the number into divisible parts, starting from 2 up to the length of the number (in terms of number of chars) (e.g. if number is 12345 can only be split in 5 parts: 1, 2, 3, 4, 5)
            for n_parts in 2..number.to_string().len() + 1 {
                if number_is_splittable(number.to_string(), n_parts) {
                    match split_number_in_n_parts(number.to_string(), n_parts as usize) {
                        Ok(parts_array) => {
                            if is_invalid_id(parts_array) {
                                solution += number;
                                // Go to next number, avoid splitting more times if already found a solution (e.g. 2222 can be split in 2 and 4 parts, but should be only counted once)
                                break;
                            }
                        }
                        Err(error) => {
                            print!("{}", error);
                            continue;
                        }
                    };
                    continue;
                }
            }
        }
    }
    println!("The solution for Puzzle 02 Part 02 is: {}", solution);
}

/// Checks if an ID is invalid. An ID is considered invalid in the case that all the parts that compose the ID (already split) are equal to each other. For example, 123123 is an invalid ID because 123 and 123 are equal.
/// # Arguments
/// * `split_id` - A vector containing all the parts that compose the full ID.
/// # Returns
/// Returns true id the ID is invalid, false otherwise
fn is_invalid_id(split_id: Vec<String>) -> bool {
    let first = &split_id[0];
    split_id.iter().all(|part| part == first)
}

/// Checks if a number can be evenly split into n_parts
/// # Arguments
/// * `number` - The number to be checked as a string
/// * `n_parts` - The number of parts to split the number into
/// # Returns
/// Returns true if the number can be evenly split into n_parts, false otherwise
fn number_is_splittable(number: String, n_parts: usize) -> bool {
    number.len() % n_parts == 0
}

/// Splits a number into n_parts and returns a vector of strings representing each part
/// # Arguments
/// * `number` - The number to be split as a string
/// * `n_parts` - The number of parts to split the number into
/// # Errors
/// Returns an error if the number cannot be evenly split into n_parts
fn split_number_in_n_parts(number: String, n_parts: usize) -> Result<Vec<String>, String> {
    let len = number.len();
    let part_size = len / n_parts as usize;
    if part_size == 0 {
        return Err("Number is too short to be split in n_parts".to_string());
    }
    let mut parts = Vec::new();
    let chars: Vec<char> = number.chars().collect();
    for i in 0..n_parts {
        let start = (i as usize) * part_size;
        let end = if i == n_parts - 1 {
            len
        } else {
            start + part_size
        };
        let part: String = chars[start..end].iter().collect();
        parts.push(part);
    }
    Ok(parts)
}
