const DEBUG_LOG_EN: bool = false;

macro_rules! puzzle_input {
    () => {
        include_str!("../resources/puzzle_02_input.csv")
    };
}

fn has_even_num_of_digits(number: String) -> bool {
    let mut num_of_digits: u32 = 0;
    for _ in number.chars() {
        num_of_digits += 1;
    }
    if (num_of_digits % 2) == 0 {
        return true;
    }
    return false;
}

#[derive(Debug)]
struct SplitNumber {
    high_part: String,
    low_part: String,
}

fn split_number_in_two(number: String) -> SplitNumber {
    let split_number_len = number.len() / 2;
    let mut high_part: String = "".to_string();
    let mut low_part: String = "".to_string();
    let mut index = 0;
    for digit in number.chars() {
        index += 1;
        if index <= split_number_len {
            high_part.push(digit);
        }
        if index > split_number_len {
            low_part.push(digit);
        }
    }
    return SplitNumber {
        high_part,
        low_part,
    };
}

pub fn puzzle_02_01() {
    let instructions = puzzle_input!();
    let mut solution: u64 = 0;
    for range_str in instructions.split(',') {
        let range_str = range_str.trim();
        if range_str.is_empty() {
            continue;
        }
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() == 2 {
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            for number in start..end + 1 {
                if has_even_num_of_digits(number.to_string()) {
                    let split_num: SplitNumber = split_number_in_two(number.to_string());
                    if split_num.low_part == split_num.high_part {
                        if DEBUG_LOG_EN {
                            println!("Found invalid ID: {}", number);
                        }
                        solution += number;
                    }
                }
            }
        }
    }
    println!("The solution for Puzzle 02 Part 01 is: {}", solution);

    let instructions = puzzle_input!();
    let mut solution: u64 = 0;
    for range_str in instructions.split(',') {
        let range_str = range_str.trim();
        if range_str.is_empty() {
            continue;
        }
        let parts: Vec<&str> = range_str.split('-').collect();
        // let mut skip_to_next_number = false;
        if parts.len() == 2 {
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            for number in start..end + 1 {
                for n_parts in 2..number.to_string().len() + 1 {
                    if number_is_splittable(number.to_string(), n_parts) {
                        // println!("{}", number);
                        match split_number_in_n_parts(number.to_string(), n_parts as u8) {
                            Ok(parts_array) => {
                                // println!("{:?}", parts_array);
                                if !parts_array.is_empty() {
                                    let first = &parts_array[0];
                                    let all_equal = parts_array.iter().all(|part| part == first);
                                    if all_equal {
                                        // Todas las partes son iguales
                                        // println!("add to solution");
                                        solution += number;
                                        break;
                                        // skip_to_next_number = true;
                                    }
                                }
                            }
                            Err(error) => {
                                print!("{}", error);
                                continue;
                            }
                        };
                        continue;
                    }
                    // if skip_to_next_number {
                    //     skip_to_next_number = false;
                    //     break;
                    // }
                }
            }
        }
    }
    println!("The solution for Puzzle 02 Part 02 is: {}", solution);
}

fn number_is_splittable(number: String, n_parts: usize) -> bool {
    if (number.len() % n_parts) == 0 {
        return true;
    }
    return false;
}

fn split_number_in_n_parts(number: String, n_parts: u8) -> Result<Vec<String>, String> {
    let len = number.len();
    let part_size = len / n_parts as usize;
    if part_size == 0 {
        return Err("Too short to divide".to_string());
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
