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
        let our_move = chars.next();
        if let (None, None) = (their_move, our_move) {
            return Err(());
        };
        Ok(Self {
            our_move: our_move.unwrap().try_into()?,
            their_move: their_move.unwrap().try_into()?,
        })
    }
}

pub fn get_total_score(lines: &Vec<String>) -> u64 {
    lines
        .iter()
        .map(|line| line.parse::<Round>().unwrap().points())
        .sum()
}
