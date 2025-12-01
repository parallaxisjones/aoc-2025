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

pub fn part_one(input: &str) -> Option<u64> {
    let start_pos: u8 = 50;
    let moves = input.split("\n").filter_map(|m| {
        if !m.is_empty() {
            let turn = MoveStr {
                value: m[1..].to_string(),
                is_neg: m.starts_with("L"),
            };
            let conv = turn.to_dialmove().ok();
            conv
        } else {
            None
        }
    });
    Some(count_zero_hits(moves, start_pos))
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
