pub struct Game {
    ball: u32,
    score: u32,
    throws: [u32; 21],
    current_throw: u32,
    current_frame: u32,
    first_throw_in_frame: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            ball: 0,
            score: 0,
            throws: [0; 21],
            current_throw: 0,
            current_frame: 1,
            first_throw_in_frame: true,
        }
    }
    pub fn score(&mut self) -> u32 {
        self.score_for_frame(self.get_current_frame() - 1)
    }
    pub fn add(&mut self, pins: u32) {
        self.throws[self.current_throw as usize] = pins;
        self.current_throw += 1;
        self.score += pins;
        self.adjust_current_frame(pins);
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
    pub fn get_current_frame(&self) -> u32 {
        self.current_frame
    }
    fn adjust_current_frame(&mut self, pins: u32) {
        if self.first_throw_in_frame {
            if pins == 10 {
                self.current_frame += 1;
            } else {
                self.first_throw_in_frame = false;
            }
        } else {
            self.first_throw_in_frame = true;
            self.current_frame += 1;
        }
        self.current_frame = std::cmp::min(11, self.current_frame);
    }
    fn strike(&self) -> bool {
        self.throws[self.ball as usize] == 10
    }
    fn next_two_balls_for_strike(&self) -> u32 {
        self.throws[(self.ball + 1) as usize] + self.throws[(self.ball + 2) as usize]
    }
    fn spare(&self) -> bool {
        (self.throws[self.ball as usize] + self.throws[(self.ball + 1) as usize]) == 10
    }
    fn next_ball_for_spare(&self) -> u32 {
        self.throws[(self.ball + 2) as usize]
    }
    fn two_balls_in_frame(&self) -> u32 {
        self.throws[self.ball as usize] + self.throws[(self.ball + 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_throws_no_mark() {
        let mut g = Game::new();
        g.add(5);
        g.add(4);
        assert_eq!(9, g.score());
        assert_eq!(2, g.get_current_frame());
    }

    #[test]
    fn test_four_throws_no_mark() {
        let mut g = Game::new();
        g.add(5);
        g.add(4);
        g.add(7);
        g.add(2);
        assert_eq!(18, g.score());
        assert_eq!(9, g.score_for_frame(1));
        assert_eq!(18, g.score_for_frame(2));
        assert_eq!(3, g.get_current_frame());
    }

    #[test]
    fn test_simple_spare() {
        let mut g = Game::new();
        g.add(3);
        g.add(7);
        g.add(3);
        assert_eq!(13, g.score_for_frame(1));
        assert_eq!(2, g.get_current_frame());
    }

    #[test]
    fn test_simple_frame_after_spare() {
        let mut g = Game::new();
        g.add(3);
        g.add(7);
        g.add(3);
        g.add(2);
        assert_eq!(13, g.score_for_frame(1));
        assert_eq!(18, g.score_for_frame(2));
        assert_eq!(18, g.score());
        assert_eq!(3, g.get_current_frame());
    }

    #[test]
    fn test_simple_strike() {
        let mut g = Game::new();
        g.add(10);
        g.add(3);
        g.add(6);
        assert_eq!(19, g.score_for_frame(1));
        assert_eq!(28, g.score());
        assert_eq!(3, g.get_current_frame());
    }

    #[test]
    fn test_perfect_game() {
        let mut g = Game::new();
        for _ in 0..12 {
            g.add(10);
        }
        assert_eq!(300, g.score());
        assert_eq!(11, g.get_current_frame());
    }

    #[test]
    fn test_end_of_array() {
        let mut g = Game::new();
        for _ in 0..9 {
            g.add(0);
            g.add(0);
        }
        g.add(2);
        g.add(8);
        g.add(10);
        assert_eq!(20, g.score());
    }

    #[test]
    fn test_sample_game() {
        let mut g = Game::new();
        g.add(1);
        g.add(4);
        g.add(4);
        g.add(5);
        g.add(6);
        g.add(4);
        g.add(5);
        g.add(5);
        g.add(10);
        g.add(0);
        g.add(1);
        g.add(7);
        g.add(3);
        g.add(6);
        g.add(4);
        g.add(10);
        g.add(2);
        g.add(8);
        g.add(6);
        assert_eq!(133, g.score());
    }

    #[test]
    fn test_heart_break() {
        let mut g = Game::new();
        for _ in 0..11 {
            g.add(10);
        }
        g.add(9);
        assert_eq!(299, g.score());
    }

    #[test]
    fn test_tenth_frame_spare() {
        let mut g = Game::new();
        for _ in 0..9 {
            g.add(10);
        }
        g.add(9);
        g.add(1);
        g.add(1);
        assert_eq!(270, g.score());
    }
}
