#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    rolls_this_frame: u16,
    pins_standing: u16,
    fill_ball: bool,
    multipliers: (u16, u16),
    score: u16,
    frames_completed: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            pins_standing: 10,
            ..Default::default()
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames_completed == 10 {
            return Err(Error::GameComplete);
        } else if pins > self.pins_standing {
            return Err(Error::NotEnoughPinsLeft);
        }
        self.rolls_this_frame += 1;
        self.pins_standing -= pins;

        self.score += pins * (1 + self.multipliers.0);
        self.multipliers = (self.multipliers.1, 0);
        if self.pins_standing == 0 && self.frames_completed < 9 {
            self.multipliers.0 += 1;
            if self.rolls_this_frame == 1 {
                self.multipliers.1 += 1;
            }
        } else if self.pins_standing == 0 && self.frames_completed == 9 {
            self.pins_standing = 10;
            self.fill_ball = true;
        }

        if (self.rolls_this_frame == 2 && !self.fill_ball)
            || self.rolls_this_frame == 3
            || self.pins_standing == 0
        {
            self.frames_completed += 1;
            self.pins_standing = 10;
            self.rolls_this_frame = 0;
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        match self.frames_completed {
            10 => Some(self.score),
            _ => None,
        }
    }
}
