pub struct Frame {
    score: u32,
}

impl Frame {
    pub fn new() -> Self {
        Frame { score: 0 }
    }
    pub fn get_score(&self) -> u32 {
        self.score
    }
    pub fn add(&mut self, pins: u32) {
        self.score += pins;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_score_no_throws() {
        let f = Frame::new();
        assert_eq!(0, f.get_score());
    }
    #[test]
    fn test_add_one_throw() {
        let mut f = Frame::new();
        f.add(5);
        assert_eq!(5, f.get_score());
    }
}
