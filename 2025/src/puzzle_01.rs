struct Knob {
    state: u32,
    zero_passes: u32,
}

impl Knob {
    fn move_right(&mut self, amount: u32) -> u32 {
        let effective_amount = amount % 100;
        if effective_amount > self.state {
            return 100 - (effective_amount - self.state);
        } else {
            return self.state - effective_amount;
        }
    }

    fn move_left(&mut self, amount: u32) -> u32 {
        let effective_amount = amount % 100;
        let result = self.state + effective_amount;
        if result > 99 {
            return result - 100;
        } else {
            return result;
        }
    }

    fn get_state(self) -> u32 {
        return self.state;
    }

    fn get_zero_passes(self) -> u32 {
        return self.zero_passes;
    }
}

fn move_knob_left(knob_state: u32, amount: u32) -> u32 {
    let effective_amount = amount % 100;
    if effective_amount > knob_state {
        return 100 - (effective_amount - knob_state);
    } else {
        return knob_state - effective_amount;
    }
}

fn move_knob_right(knob_state: u32, amount: u32) -> u32 {
    let effective_amount = amount % 100;
    let result = knob_state + effective_amount;
    if result > 99 {
        return result - 100;
    } else {
        return result;
    }
}

pub fn puzzle_01_01() {
    let instructions_to_decode = include_str!("puzzle_01_input.txt");

    // The knob goes from 0 to 99, it has 100 positions. A move of 5 left from 0 results in -5, and a move of 10 from -5 results in 5

    let mut knob_state: u32 = 50;
    let mut password: u32 = 0;

    for instruction in instructions_to_decode.lines() {
        let knob_direction = &instruction[0..1];
        let amount: u32 = instruction[1..].parse().expect("Not a valid number!");
        println!("Instruction: {knob_direction} by {amount}");
        if knob_direction == "R" {
            knob_state = move_knob_right(knob_state, amount);
        }
        if knob_direction == "L" {
            knob_state = move_knob_left(knob_state, amount);
        }
        if knob_state == 0 {
            password += 1;
        }
    }
    println!("The password for Puzzle 01 Part 01 is: {password}");
}

pub fn puzzle_01_02() {}
