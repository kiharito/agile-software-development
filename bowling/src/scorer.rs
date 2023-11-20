pub struct Scorer {
    ball: u32,
    throws: [u32; 21],
    current_throw: u32,
}

impl Scorer {
    pub fn new() -> Self {
        Scorer {
            ball: 0,
            throws: [0; 21],
            current_throw: 0,
        }
    }
    pub fn add_throw(&mut self, pins: u32) {
        self.throws[self.current_throw as usize] = pins;
        self.current_throw += 1;
    }
    pub fn score_for_frame(&mut self, frame: u32) -> u32 {
        self.ball = 0;
        let mut score = 0;
        for _current_frame in 0..frame {
            if self.strike() {
                score += 10 + self.next_two_balls_for_strike();
                self.ball += 1;
            } else if self.spare() {
                score += 10 + self.next_ball_for_spare();
                self.ball += 2;
            } else {
                score += self.two_balls_in_frame();
                self.ball += 2;
            }
        }
        score
    }
    fn strike(&self) -> bool {
        self.throws[self.ball as usize] == 10
    }
    fn spare(&self) -> bool {
        (self.throws[self.ball as usize] + self.throws[(self.ball + 1) as usize]) == 10
    }
    fn next_two_balls_for_strike(&self) -> u32 {
        self.throws[(self.ball + 1) as usize] + self.throws[(self.ball + 2) as usize]
    }
    fn next_ball_for_spare(&self) -> u32 {
        self.throws[(self.ball + 2) as usize]
    }
    fn two_balls_in_frame(&self) -> u32 {
        self.throws[self.ball as usize] + self.throws[(self.ball + 1) as usize]
    }
}
