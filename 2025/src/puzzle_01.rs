struct Knob {
    state: u32,
    zero_passes: u32,
}

impl Knob {
    fn new(initial_state: u32) -> Knob {
        return Knob {
            state: initial_state,
            zero_passes: 0,
        };
    }

    fn move_right(&mut self, amount: u32) {
        if amount > 99 {
            self.zero_passes += amount / 100;
        }
        let effective_amount = amount % 100;
        if effective_amount > self.state {
            self.zero_passes += 1;
            self.state = 100 - (effective_amount - self.state);
        } else {
            self.state = self.state - effective_amount;
        }
    }

    fn move_left(&mut self, amount: u32) {
        if amount > 99 {
            self.zero_passes += amount / 100;
        }
        let effective_amount = amount % 100;
        let result = self.state + effective_amount;
        if result > 99 {
            self.zero_passes += 1;
            self.state = result - 100;
        } else {
            self.state = result;
        }
    }

    fn get_state(&self) -> u32 {
        return self.state;
    }

    fn get_zero_passes(&self) -> u32 {
        return self.zero_passes;
    }
}

pub fn puzzle_01_01() {
    let instructions_to_decode = include_str!("puzzle_01_input.txt");
    let mut knob = Knob::new(50);
    let mut password: u32 = 0;

    for instruction in instructions_to_decode.lines() {
        let knob_direction = &instruction[0..1];
        let amount: u32 = instruction[1..].parse().expect("Not a valid number!");
        println!("Instruction: {knob_direction} by {amount}");
        if knob_direction == "R" {
            knob.move_right(amount);
        }
        if knob_direction == "L" {
            knob.move_left(amount);
        }
        if knob.get_state() == 0 {
            password += 1;
        }
    }
    println!("The password for Puzzle 01 Part 01 is: {password}");
}

pub fn puzzle_01_02() {
    let instructions_to_decode = include_str!("puzzle_01_input.txt");
    let mut knob = Knob::new(50);
    let mut password: u32 = 0;

    for instruction in instructions_to_decode.lines() {
        let knob_direction = &instruction[0..1];
        let amount: u32 = instruction[1..].parse().expect("Not a valid number!");
        println!("Instruction: {knob_direction} by {amount}");
        if knob_direction == "R" {
            knob.move_right(amount);
        }
        if knob_direction == "L" {
            knob.move_left(amount);
        }
        if knob.get_state() == 0 {
            password += 1;
        }
    }
    password += knob.get_zero_passes();
    println!("The password for Puzzle 01 Part 02 is: {password}");
}
