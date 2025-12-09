use std::ops::{Index, IndexMut};
use crate::Vector;

pub type Row<TCell> = Vec<TCell>;

#[derive(Clone, PartialEq)]
pub struct Grid<TCell> {
    size: Vector,
    pub rows: Vec<Row<TCell>>,
}

impl<TCell> Grid<TCell> {
    pub fn new(rows: Vec<Row<TCell>>) -> Grid<TCell> {
        let height = rows.len();
        let width = if height > 0 { rows[0].len() } else { 0 };
        let size = Vector {
            x: width as i64,
            y: height as i64,
        };

        Grid { rows, size }
    }

    pub fn with_capacity(size: Vector, default_value: TCell) -> Grid<TCell>
    where
        TCell: Clone,
    {
        let mut rows: Vec<Row<TCell>> = Vec::with_capacity(size.y as usize);
        for _ in 0..size.y {
            let mut row = Vec::with_capacity(size.x as usize);
            for _ in 0..size.x {
                row.push(default_value.clone());
            }

            rows.push(row);
        }
        Grid { rows, size }
    }

    pub fn get(&self, position: &Vector) -> Option<&TCell> {
        self.rows.get(position.y as usize)?.get(position.x as usize)
    }

    pub fn get_mut(&mut self, position: &Vector) -> Option<&mut TCell> {
        self.rows
            .get_mut(position.y as usize)?
            .get_mut(position.x as usize)
    }

    pub fn size(&self) -> &Vector {
        &self.size
    }

    pub fn is_inside(&self, pos: &Vector) -> bool {
        if pos.x < 0 || pos.y < 0 {
            return false;
        }

        if pos.x >= self.size.x || pos.y >= self.size.y {
            return false;
        }

        true
    }
}

impl<TCell> Index<&Vector> for Grid<TCell> {
    type Output = TCell;

    fn index(&self, index: &Vector) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<TCell> Index<Vector> for Grid<TCell> {
    type Output = TCell;

    fn index(&self, index: Vector) -> &Self::Output {
        self.get(&index).expect("index out of bounds")
    }
}

impl<TCell> IndexMut<&Vector> for Grid<TCell> {
    fn index_mut(&mut self, index: &Vector) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}

impl<TCell> IndexMut<Vector> for Grid<TCell> {
    fn index_mut(&mut self, index: Vector) -> &mut Self::Output {
        self.get_mut(&index).expect("index out of bounds")
    }
}
