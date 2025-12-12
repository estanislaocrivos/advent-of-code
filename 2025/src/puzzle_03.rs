use std::usize;

const DEBUG_LOG_EN: bool = false;
const TEST_EXAMPLE: bool = false;

macro_rules! puzzle_input {
    () => {
        if TEST_EXAMPLE {
            include_str!("../resources/puzzle_03_input_example.txt")
        } else {
            include_str!("../resources/puzzle_03_input.txt")
        }
    };
}

// For this part a simple double-iteration works. Another approach would be to take the biggest numbers in the bank and arrange them big * 10 + small (approach followed on part 2)
pub fn puzzle_03_01() {
    let battery_banks = puzzle_input!();
    let mut total_joltage: usize = 0;
    let mut max_joltage: usize = 0;

    for bank in battery_banks.lines() {
        for (i, batt_digit_1) in bank.chars().enumerate() {
            for batt_digit_2 in bank.chars().skip(i + 1) {
                if let (Some(d1), Some(d2)) = (batt_digit_1.to_digit(10), batt_digit_2.to_digit(10))
                {
                    let joltage: usize = d1 as usize * 10 + d2 as usize;
                    if joltage > max_joltage {
                        max_joltage = joltage;
                    }
                }
            }
        }
        if DEBUG_LOG_EN {
            println!("The highest joltage for bank {} is {}", bank, max_joltage);
        }
        total_joltage += max_joltage;
        max_joltage = 0;
    }

    println!(
        "The total joltage for Puzzle 03 Part 01 is: {}",
        total_joltage
    );
}

pub fn puzzle_03_02() {
    let battery_banks = puzzle_input!();
    let mut total_joltage: u64 = 0;

    for bank in battery_banks.lines() {
        let digits: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();

        if digits.len() < 12 {
            // If there are not enough digits use them all
            let joltage: u64 = digits.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);
            total_joltage += joltage;
            continue;
        }

        let mut selected = Vec::new();
        let mut start_pos = 0;

        for digits_remaining in (1..=12).rev() {
            // Always leave (digits_remaining - 1) positions
            let search_end = digits.len() - (digits_remaining - 1);

            // Look for the highest in the range [start_pos, search_end)
            let mut best_digit = 0;
            let mut best_pos = start_pos;

            for i in start_pos..search_end {
                if digits[i] > best_digit {
                    best_digit = digits[i];
                    best_pos = i;
                }
            }
            if DEBUG_LOG_EN {
                println!(
                    "Iteration {}: Searching from {} to {} (remaining: {})",
                    13 - digits_remaining,
                    start_pos,
                    search_end - 1,
                    digits_remaining
                );
                println!("Found {} at position {}", best_digit, best_pos);
            }
            selected.push(best_digit);
            start_pos = best_pos + 1;
        }

        let max_joltage: u64 = selected.iter().fold(0u64, |acc, &d| acc * 10 + d as u64);

        if DEBUG_LOG_EN {
            println!("The highest joltage for bank {} is {}", bank, max_joltage);
        }
        total_joltage += max_joltage;
    }

    println!(
        "The total joltage for Puzzle 03 Part 02 is: {}",
        total_joltage
    );
}
