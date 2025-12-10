use std::usize;

const DEBUG_LOG_EN: bool = false;

macro_rules! puzzle_input {
    () => {
        include_str!("../resources/puzzle_03_input.txt")
    };
}

pub fn puzzle_03_01() {
    let battery_banks = puzzle_input!();
    let mut total_joltage: usize = 0;
    let mut higher_joltage: usize = 0;
    for bank in battery_banks.lines() {
        for (i, batt_digit_1) in bank.chars().enumerate() {
            for batt_digit_2 in bank.chars().skip(i + 1) {
                if let (Some(d1), Some(d2)) = (batt_digit_1.to_digit(10), batt_digit_2.to_digit(10))
                {
                    let joltage: usize = d1 as usize * 10 + d2 as usize;
                    if joltage > higher_joltage {
                        higher_joltage = joltage;
                    }
                }
            }
        }
        if DEBUG_LOG_EN {
            println!("The higher joltage for bank {} is {}", bank, higher_joltage);
        }
        total_joltage += higher_joltage;
        higher_joltage = 0;
    }
    println!(
        "The total joltage for Puzzle 03 Part 01 is: {}",
        total_joltage
    );
}
