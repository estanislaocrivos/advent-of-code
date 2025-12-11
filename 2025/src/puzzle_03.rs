use std::usize;

const DEBUG_LOG_EN: bool = true;
const TEST_EXAMPLE: bool = true;

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
