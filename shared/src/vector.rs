use std::ops;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, dir: Vector) -> Vector {
        Vector {
            x: self.x + dir.x,
            y: self.y + dir.y,
        }
    }
}
