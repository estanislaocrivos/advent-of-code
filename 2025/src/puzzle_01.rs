const DEBUG_LOG_EN: bool = true;

struct Knob {
    /// Current position of the knob, in the range [0, 99].
    state: u32,

    /// Counts how many times the knob has passed through zero.
    zero_passes: u32,

    /// Counts how many times the knob has landed on zero.
    zero_position: u32,
}

impl Knob {
    /// Creates a new Knob with the specified initial state.
    /// # Arguments
    /// * `initial_state` - The initial position of the knob, in the range [0, 99].
    fn new(initial_state: u32) -> Knob {
        return Knob {
            state: initial_state,
            zero_passes: 0,
            zero_position: 0,
        };
    }

    /// Moves the knob by the specified amount, wrapping around if necessary.
    /// The amount can be positive (to the right) or negative (to the left).
    /// The state is always kept within the range [0, 99].
    /// # Arguments
    /// * `amount` - The amount to move the knob by.
    ///
    /// # Examples
    ///
    /// Another implementation without rem_euclid:
    /// ```
    /// fn move_knob(&mut self, amount: i32) {
    ///     let mut result = self.state as i32 + amount;
    ///     while result < 0 {
    ///         result += 100;
    ///     }
    ///     while result >= 100 {
    ///        result -= 100;
    ///     }
    ///     self.state = result as u32;
    ///  }
    /// ```
    fn move_knob(&mut self, amount: i32) {
        let new = (self.state as i32 + amount).rem_euclid(100); // Ensures wrapping within [0, 99]
        self.state = new as u32;
    }

    fn move_left(&mut self, amount: u32) {
        self.zero_passes += amount / 100;
        let effective_amount = amount % 100;
        if effective_amount == 0 {
            return;
        }
        if DEBUG_LOG_EN {
            print!(
                "Current state: {}. Moving left by {}...",
                self.state, effective_amount,
            );
        }
        self.move_knob(-(effective_amount as i32));
        if self.state == 0 {
            self.zero_position += 1;
        }
        if DEBUG_LOG_EN {
            print!("New state: {}.\n\n", self.state);
        }
    }

    fn move_right(&mut self, amount: u32) {
        self.zero_passes += amount / 100;
        let effective_amount = amount % 100;
        if effective_amount == 0 {
            return;
        }
        if DEBUG_LOG_EN {
            print!(
                "Current state: {}. Moving right by {}...",
                self.state, effective_amount
            );
        }
        self.move_knob(effective_amount as i32);
        if self.state == 0 {
            self.zero_position += 1;
        }
        if DEBUG_LOG_EN {
            print!("New state: {}.\n\n", self.state);
        }
    }

    fn get_zero_passes(&self) -> u32 {
        return self.zero_passes;
    }

    fn get_zero_position(&self) -> u32 {
        return self.zero_position;
    }
}

pub fn puzzle_01_01() {
    let instructions_to_decode = include_str!("puzzle_01_input.txt");
    let mut knob = Knob::new(50);

    for instruction in instructions_to_decode.lines() {
        let knob_direction = &instruction[0..1];
        let amount: u32 = instruction[1..].parse().expect("Not a valid number!");
        if DEBUG_LOG_EN {
            println!("Instruction: {knob_direction} by {amount}");
        }
        if knob_direction == "R" {
            knob.move_right(amount);
        }
        if knob_direction == "L" {
            knob.move_left(amount);
        }
    }
    println!(
        "The password for Puzzle 01 Part 01 is: {}",
        knob.get_zero_position()
    );
}

pub fn puzzle_01_02() {
    let instructions_to_decode = include_str!("puzzle_01_input.txt");
    let mut knob = Knob::new(50);

    for instruction in instructions_to_decode.lines() {
        let knob_direction = &instruction[0..1];
        let amount: u32 = instruction[1..].parse().expect("Not a valid number!");
        if DEBUG_LOG_EN {
            println!("Instruction: {knob_direction} by {amount}");
        }
        if knob_direction == "R" {
            knob.move_right(amount);
        }
        if knob_direction == "L" {
            knob.move_left(amount);
        }
    }
    println!(
        "The password for Puzzle 01 Part 02 is: {}",
        knob.get_zero_passes() + knob.get_zero_position()
    );
}
