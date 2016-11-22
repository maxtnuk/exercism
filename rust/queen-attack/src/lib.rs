pub struct ChessPosition {
    position: (u32, u32),
}
pub struct Queen {
    current_postion: ChessPosition,
}
impl ChessPosition {
    pub fn new(x: i32, y: i32) -> Result<ChessPosition, &'static str> {
        match (x, y) {
            (0...7, 0...7) => Ok(ChessPosition { position: (x as u32, y as u32) }),
            (_, _) => Err("Invalid Position"),
        }
    }
}
impl Queen {
    pub fn new(pos: ChessPosition) -> Queen {
        Queen { current_postion: pos }
    }
    pub fn can_attack(&self, oppose: &Queen) -> bool {
        let vec_x = oppose.current_postion.position.0 as i32 -
                    self.current_postion.position.0 as i32;
        let vec_y = oppose.current_postion.position.1 as i32 -
                    self.current_postion.position.1 as i32;
        match (vec_x, vec_y) {
            (0, _) => true,
            (_, 0) => true,
            (_, _) => {
                match vec_x / vec_y {
                    1 | -1 => true,
                    _ => false,
                }
            }
        }
    }
}
