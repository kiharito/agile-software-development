pub struct Game {
    score: u32,
    throws: [u32; 21],
    current_throw: u32,
    current_frame: u32,
    is_first_throw: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            throws: [0; 21],
            current_throw: 0,
            current_frame: 1,
            is_first_throw: true,
        }
    }
    pub fn score(&self) -> u32 {
        self.score
    }
    pub fn add(&mut self, pins: u32) {
        self.throws[self.current_throw as usize] = pins;
        self.current_throw += 1;
        self.score += pins;
        self.adjust_current_frame();
    }
    pub fn score_for_frame(&self, frame: u32) -> u32 {
        let mut score = 0;
        let mut ball = 0;
        for _current_frame in 0..frame {
            let first_throw = self.throws[ball as usize];
            ball += 1;
            let second_throw = self.throws[ball as usize];
            ball += 1;
            let frame_score = first_throw + second_throw;
            if frame_score == 10 {
                score += frame_score + self.throws[ball as usize];
            } else {
                score += frame_score;
            }
        }
        score
    }
    pub fn get_current_frame(&self) -> u32 {
        self.current_frame
    }
    fn adjust_current_frame(&mut self) {
        if self.is_first_throw {
            self.is_first_throw = false;
        } else {
            self.is_first_throw = true;
            self.current_frame += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_throw() {
        let mut g = Game::new();
        g.add(5);
        assert_eq!(5, g.score());
        assert_eq!(1, g.get_current_frame());
    }

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
        assert_eq!(3, g.get_current_frame());
    }
}
