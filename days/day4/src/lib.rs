#[derive(Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub struct Shift {
    pub x: i8,
    pub y: i8,
}

impl std::ops::Add<Shift> for Position {
    type Output = Position;

    fn add(self, rhs: Shift) -> Self::Output {
        let Self { x, y } = self;
        Self {
            x: x + rhs.x as i32,
            y: y + rhs.y as i32,
        }
    }
}

impl std::ops::Mul<i8> for Shift {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self::Output {
        let Self { x, y } = self;
        Self {
            x: x * rhs,
            y: y * rhs,
        }
    }
}

pub struct Grid(pub Vec<String>);
impl Grid {
    pub fn get(&self, Position { x, y }: Position) -> Option<char> {
        let y: usize = y.try_into().ok()?;
        let x: usize = x.try_into().ok()?;
        let row = self.0.get(y)?;
        row.chars().nth(x)
    }
}
