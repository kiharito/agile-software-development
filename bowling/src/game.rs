use crate::scorer::Scorer;

pub struct Game {
    current_frame: u32,
    first_throw_in_frame: bool,
    scorer: Scorer,
}

impl Game {
    pub fn new() -> Self {
        Game {
            current_frame: 1,
            first_throw_in_frame: true,
            scorer: Scorer::new(),
        }
    }
    pub fn score(&mut self) -> u32 {
        self.score_for_frame(self.current_frame)
    }
    pub fn add(&mut self, pins: u32) {
        self.scorer.add_throw(pins);
        self.adjust_current_frame(pins);
    }
    fn adjust_current_frame(&mut self, pins: u32) {
        if self.last_ball_in_frame(pins) {
            self.advance_frame();
        } else {
            self.first_throw_in_frame = false;
        }
    }
    fn last_ball_in_frame(&self, pins: u32) -> bool {
        self.strike(pins) || !self.first_throw_in_frame
    }
    fn strike(&self, pins: u32) -> bool {
        self.first_throw_in_frame && pins == 10
    }
    fn advance_frame(&mut self) {
        self.current_frame = std::cmp::min(10, self.current_frame + 1)
    }
    pub fn score_for_frame(&mut self, frame: u32) -> u32 {
        self.scorer.score_for_frame(frame)
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
    }

    #[test]
    fn test_simple_spare() {
        let mut g = Game::new();
        g.add(3);
        g.add(7);
        g.add(3);
        assert_eq!(13, g.score_for_frame(1));
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
    }

    #[test]
    fn test_simple_strike() {
        let mut g = Game::new();
        g.add(10);
        g.add(3);
        g.add(6);
        assert_eq!(19, g.score_for_frame(1));
        assert_eq!(28, g.score());
    }

    #[test]
    fn test_perfect_game() {
        let mut g = Game::new();
        for _ in 0..12 {
            g.add(10);
        }
        assert_eq!(300, g.score());
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
