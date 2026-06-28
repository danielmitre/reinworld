use std::fmt::{Debug, Write};

const HEIGHT: usize = 30;
const WIDTH: usize = 80;

#[derive(Copy, Clone, Default)]
enum MapPosition {
    #[default]
    Empty,
    Tree,
}

impl Debug for MapPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapPosition::Empty => f.write_char(' '),
            MapPosition::Tree => f.write_char('T'),
        }
    }
}

struct Map {
    positions: [[MapPosition; WIDTH]; HEIGHT],
}

impl Map {
    fn new() -> Self {
        Map {
            positions: [[MapPosition::Empty; WIDTH]; HEIGHT],
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.positions {
            for col in line {
                col.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn main() {
    let mut map: Map = Map::new();
    for row in   map.positions.iter_mut().step_by(7) {
        for cell in row.iter_mut().step_by(13) {
            *cell = MapPosition::Tree;
        }
    }
    println!("{:?}", map)
}
