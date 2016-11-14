// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
    pos: (isize,isize),
    direction: Direction,
}

impl Robot {
    #[allow(unused_variables)]
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot{pos: (x,y),direction: d}
    }

    pub fn turn_right(self) -> Self {
        Robot{pos: self.pos,
            direction: match self.direction{
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::North => Direction::East,
        }}
    }

    pub fn turn_left(self) -> Self {
        Robot{pos: self.pos,
            direction: match self.direction{
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                Direction::North => Direction::West,
        }}
    }

    pub fn advance(self) -> Self {
        let mut x: isize=self.pos.0;
        let mut y: isize=self.pos.1;
        match self.direction{
            Direction::East => {x+=1;},
            Direction::South => {y-=1;},
            Direction::West => {x-=1;},
            Direction::North => {y+=1;},
        };
        Robot{pos: (x,y),direction: self.direction}
    }
    pub fn instructions(self, instructions: &str) -> Self {
        let mut result=self;
        for temp in instructions.chars()
        {
            result=
            match temp{
                'L'=> result.turn_left(),
                'R'=> result.turn_right(),
                'A'=> result.advance(),
                _ => result
            };
        }
        result
    }
    pub fn position(&self) -> (isize, isize) {
        self.pos
    }
    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
