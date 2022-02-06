#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    is_completed: bool,
    current_frame: usize,
    frames: [[Option<u16>; 3]; 10],
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            is_completed: false,
            current_frame: 0,
            frames: [[None; 3]; 10],
        }
    }

    fn should_advance_frame(&self, round: usize, frame_score: u16) -> bool {
        match (round, frame_score, self.is_last_frame()) {
            (0, 10, false) => true,
            (1, 0..=9, _) => true,
            (1, _, false) => true,
            (2, _, _) => true,
            _ => false,
        }
    }

    fn advance_frame(&mut self) {
        if self.is_last_frame() {
            self.is_completed = true;
        } else {
            self.current_frame += 1
        }
    }

    fn add_frame_score(&mut self, round: usize, current_score: u16, pins: u16) {
        self.frames[self.current_frame][round] = Some(pins);
        if self.should_advance_frame(round, current_score + pins) {
            self.advance_frame();
        }
    }

    fn is_last_frame(&self) -> bool {
        self.current_frame == self.frames.len() - 1
    }

    fn is_roll_valid(&self, round: usize, current_score: u16, pins: u16) -> bool {
        match round {
            0 => pins <= 10,
            1 => pins + (current_score % 10) <= 10,
            2 => (current_score % 10) + pins <= 10,
            _ => false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_completed {
            return Err(Error::GameComplete);
        }
        let result = match self.frames[self.current_frame] {
            [None, _, _] if self.is_roll_valid(0, 0, pins) => Ok(self.add_frame_score(0, 0, pins)),
            [Some(a), None, _] if self.is_roll_valid(1, a, pins) => {
                Ok(self.add_frame_score(1, a, pins))
            }
            [Some(a), Some(b), None] if self.is_roll_valid(2, a + b, pins) => {
                Ok(self.add_frame_score(2, a + b, pins))
            }
            _ => Err(Error::NotEnoughPinsLeft),
        };
        result
    }

    pub fn current_frame(&self) -> usize {
        self.current_frame
    }
    pub fn score(&self) -> Option<u16> {
        if !self.is_completed {
            return None;
        }

        let mut score: u16 = 0;

        for i in 0..10 {
            let (mut frame_score, mut carry) = match self.frames[i] {
                [Some(a), None, None] => (a, 2),
                [Some(a), Some(b), None] => (a + b, if a + b == 10 { 1 } else { 0 }),
                [Some(a), Some(b), Some(c)] => (a + b + c, 0),
                _ => panic!("Invalid score"),
            };

            for j in i + 1..=i + carry {
                match (self.frames[j], carry) {
                    ([Some(a), Some(b), _], 2) => {
                        frame_score += a + b;
                        carry -= 2;
                    }
                    ([Some(a), None, _], 2) => {
                        frame_score += a;
                        carry -= 1;
                    }
                    ([Some(a), _, _], 1) => {
                        frame_score += a;

                        carry -= 1;
                    }
                    _ => (),
                }

                if carry == 0 {
                    break;
                }
            }
            score += frame_score;
        }

        Some(score)
    }
}
