pub struct Game {
    score: u32,
    throws: [u32; 21],
    current_throw: u32,
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0,
            throws: [0; 21],
            current_throw: 0,
        }
    }
    pub fn score(&self) -> u32 {
        self.score
    }
    pub fn add(&mut self, pins: u32) {
        self.throws[self.current_throw as usize] = pins;
        self.current_throw += 1;
        self.score += pins;
    }
    pub fn score_for_frame(&self, frame: u32) -> u32 {
        let mut score = 0;
        let mut ball = 0;
        for _current_frame in 0..frame {
            let first_throw = self.throws[ball as usize];
            ball += 1;
            let second_throw = self.throws[ball as usize];
            ball += 1;
            score += first_throw + second_throw;
        }
        score
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
    }

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
}
