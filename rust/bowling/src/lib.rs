pub struct BowlingGame {
    fram_pins: Vec<u32>,
    score: u32,
}
impl BowlingGame {
    pub fn new() -> BowlingGame {
        BowlingGame {
            fram_pins: Vec::new(),
            score: 0,
        }
    }
    pub fn roll(&self, pins: i32) -> Result<BowlingGame, &'static str> {
        match pins {
            0...10 => Ok(asfd),
            _ => Err("wrong number of pins"),
        }
    }
}
