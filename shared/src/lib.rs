#[derive(Clone, Copy, PartialEq)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub type Row<TCell> = Vec<TCell>;

#[derive(Clone, PartialEq)]
pub struct Grid<TCell>
{
    pub rows: Vec<Row<TCell>>,
}

impl<TCell> Grid<TCell>
{
    pub fn new(rows: Vec<Row<TCell>>) -> Grid<TCell> {
        Grid { rows }
    }

    pub fn get(&self, position: &Position) -> Option<&TCell> {
        self.rows.get(position.y as usize)?.get(position.x as usize)
    }
    
    pub fn get_mut(&mut self, position: &Position) -> Option<&mut TCell> {
        self.rows.get_mut(position.y as usize)?.get_mut(position.x as usize)
    }
}
