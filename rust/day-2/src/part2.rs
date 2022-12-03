use std::str::FromStr;

use crate::common::{Move, Outcome};

#[derive(Debug, Clone, Copy)]
pub struct Round {
    pub our_move: Move,
    pub their_move: Move,
}

impl Round {
    fn outcome(self) -> Outcome {
        self.our_move.outcome(self.their_move)
    }

    pub fn points(self) -> u64 {
        (self.our_move as u64) + (self.outcome() as u64)
    }
}

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let their_move = chars.next();
        chars.next();
        let desired_outcome = chars.next();
        if let (None, None) = (their_move, desired_outcome) {
            return Err(());
        };
        let their_move = Move::try_from(their_move.unwrap())?;
        let desired_outcome = Outcome::try_from(desired_outcome.unwrap())?;
        let our_move = desired_outcome.matching_move(their_move);
        let s = Self {
            our_move: our_move,
            their_move: their_move,
        };
        Ok(s)
    }
}

pub fn get_total_score(lines: &Vec<String>) -> u64 {
    lines
        .iter()
        .map(|line| line.parse::<Round>().unwrap().points())
        .sum()
}
