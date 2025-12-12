use std::usize;

const DEBUG_LOG_EN: bool = true;
const TEST_EXAMPLE: bool = true;

macro_rules! puzzle_input {
    () => {
        if TEST_EXAMPLE {
            include_str!("../resources/puzzle_04_input_example.txt")
        } else {
            include_str!("../resources/puzzle_04_input.txt")
        }
    };
}

pub fn puzzle_04_01() {}

pub fn puzzle_04_02() {}
