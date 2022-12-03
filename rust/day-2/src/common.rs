#[derive(Debug, Clone, Copy)]
pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl TryFrom<char> for Outcome {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Outcome::*;
        match value {
            'X' => Ok(Win),
            'Y' => Ok(Draw),
            'Z' => Ok(Loss),
            _ => Err(()),
        }
    }
}

impl Outcome {
    pub fn matching_move(self, other: Move) -> Move {
        use Outcome::*;
        match self {
            Win => other.winning_move(),
            Draw => other.drawing_move(),
            Loss => other.losing_move(),
        }
    }
}

impl TryFrom<char> for Move {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Move::*;
        match value {
            'A' | 'X' => Ok(Rock),
            'B' | 'Y' => Ok(Paper),
            'C' | 'Z' => Ok(Scissors),
            _ => Err(()),
        }
    }
}
impl Move {
    fn beats(self, other: Move) -> bool {
        use Move::*;
        match (self, other) {
            (Rock, Scissors) => true,
            (Paper, Rock) => true,
            (Scissors, Paper) => true,
            _ => false,
        }
    }

    fn winning_move(self) -> Move {
        use Move::*;
        match self {
            Rock => Scissors,
            Scissors => Paper,
            Paper => Rock,
        }
    }

    fn losing_move(self) -> Move {
        use Move::*;
        match self {
            Paper => Scissors,
            Scissors => Rock,
            Rock => Paper,
        }
    }

    fn drawing_move(self) -> Move {
        self
    }

    pub fn outcome(self, other: Move) -> Outcome {
        use Outcome::*;
        if self.beats(other) {
            return Win;
        }
        if other.beats(self) {
            return Loss;
        }
        Draw
    }
}
