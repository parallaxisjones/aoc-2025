advent_of_code::solution!(1);

pub trait Monoid: Sized {
    /// Identity element
    fn empty() -> Self;

    /// Binary operation (must be associative)
    fn combine(self, other: Self) -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DialMove(u8);

impl DialMove {
    /// Create a dial move from any signed integer (positive = clockwise, negative = counter-clockwise)
    pub fn new(delta: i32) -> Self {
        //bring it into [0,99]
        let clicks = delta.rem_euclid(100) as u8;
        DialMove(clicks)
    }

    /// Apply this move to a dial position in [0, 99]
    pub fn apply_to(self, position: u8) -> u8 {
        debug_assert!(position < 100);
        (position + self.0) % 100
    }
    /// Get the underlying number of clicks
    pub fn clicks(self) -> u8 {
        self.0
    }
}

// implement the monoid trait for a DialMove
impl Monoid for DialMove {
    fn empty() -> Self {
        DialMove(0)
    }

    fn combine(self, other: Self) -> Self {
        // composition of moves = add clicks mod 100
        DialMove((self.0 + other.0) % 100)
    }
}

#[derive(Debug)]
struct MoveStr {
    value: String, // e.g. "15"
    is_neg: bool,  // true = negative
}

impl MoveStr {
    pub fn to_i32(&self) -> Result<i32, std::num::ParseIntError> {
        let n: i32 = self.value.parse()?;
        match self.is_neg {
            true => Ok(-n),
            false => Ok(n),
        }
    }

    pub fn to_dialmove(&self) -> Result<DialMove, std::num::ParseIntError> {
        let signed = self.to_i32()?;
        Ok(DialMove::new(signed))
    }
}

pub fn count_zero_hits<I>(moves: I, start_position: u8) -> u64
where
    I: IntoIterator<Item = DialMove>,
{
    let mut pos = start_position;
    let mut hits = 0u64;

    for m in moves {
        pos = m.apply_to(pos);
        if pos == 0 {
            hits += 1;
        }
    }

    hits
}

// CODE INTENT: two part accumulator to count total passes using moves as an i32 and a start
// position
pub fn count_zero_passes<I>(moves: I, start_position: u8) -> u64
where
    I: IntoIterator<Item = i32>,
{
    let mut pos = start_position;
    let mut total_passes = 0u64;

    for delta in moves {
        let (new_pos, passes) = zero_passes_for_move(pos, delta);
        pos = new_pos;
        total_passes += passes;
    }

    total_passes
}

//CODE INTENT: take a position and the new change to apply and return both the new position we'll
//land on after the update to the dial but also the number of times we will pass zero.
pub fn zero_passes_for_move(pos: u8, delta: i32) -> (u8, u64) {
    //get the magnitude of the delta -- this is the number of steps we are taking, irrespective of
    //direction
    let steps = delta.abs() as u64;

    //to get the full turns of the dial, which will automatically pass zero we will divide the
    //number of steps by 100
    let full_turns = steps / 100;

    //we also need to consider the partial turns possible for instances like 376,we have 3 full
    //turns and then some partial turn -- this may or may not pass the 0 again depending on where
    //on the dial we started from.So we need to handle:
    //the clockwise case, eg. if we have R333 starting for a position like 98
    //the anti-clockwise case, left turns
    //no turns
    let rem = (steps % 100) as u8;

    let extra_cross = if delta > 0 {
        //clockwise
        (pos as u16 + rem as u16) >= 100
    } else if delta < 0 {
        //counter-clockwise
        delta < 0 && pos > 0 && rem >= pos
    } else {
        false
    };

    let passes = full_turns + if extra_cross { 1 } else { 0 };
    let updated_position = DialMove::new(delta).apply_to(pos);

    (updated_position, passes)
}

pub fn part_one(input: &str) -> Option<u64> {
    let start_pos: u8 = 50;
    let moves = input.split("\n").filter_map(|m| {
        if !m.is_empty() {
            let turn = MoveStr {
                value: m[1..].to_string(),
                is_neg: m.starts_with("L"),
            };

            turn.to_dialmove().ok()
        } else {
            None
        }
    });
    Some(count_zero_hits(moves, start_pos))
}

pub fn part_two(input: &str) -> Option<u64> {
    let start_pos: u8 = 50;
    let moves = input.lines().filter_map(|m| {
        if m.is_empty() {
            return None;
        }
        let turn = MoveStr {
            value: m[1..].to_string(),
            is_neg: m.starts_with("L"),
        };

        turn.to_i32().ok()
    });
    Some(count_zero_passes(moves, start_pos))
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
    //
    #[test]
    fn zero_passes_simple_cases() {
        // Clockwise examples
        assert_eq!(zero_passes_for_move(98, 5).1, 1); // 98 -> 3, hits 0 once
        assert_eq!(zero_passes_for_move(10, 5).1, 0); // 10 -> 15, no 0

        // Counter-clockwise examples
        assert_eq!(zero_passes_for_move(5, -5).1, 1); // 5 -> 0, hits 0 once
        assert_eq!(zero_passes_for_move(5, -4).1, 0); // 5 -> 1, no 0
        assert_eq!(zero_passes_for_move(0, -5).1, 0); // 0 -> 95, no 0 in partial
    }
}
