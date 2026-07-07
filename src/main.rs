use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::LazyLock;

const HEIGHT: usize = 30;
const WIDTH: usize = 80;

#[derive(Copy, Clone, Default)]
enum MapPosition {
    #[default]
    Empty,
    Tree,
}

impl MapPosition {
    fn as_char(self) -> char {
        match self {
            MapPosition::Empty => ' ',
            MapPosition::Tree => 'T',
        }
    }
}

struct Map {
    positions: [[MapPosition; WIDTH]; HEIGHT],
}

impl Map {
    fn new() -> Self {
        Self {
            positions: [[MapPosition::Empty; WIDTH]; HEIGHT],
        }
    }

    fn generate() -> Self {
        let mut map = Self::new();
        for row in map.positions.iter_mut().step_by(7) {
            for cell in row.iter_mut().step_by(13) {
                *cell = MapPosition::Tree;
            }
        }
        map
    }

    fn render(&self) -> String {
        let mut output = String::new();
        for row in self.positions {
            for cell in row {
                output.push(cell.as_char());
            }
            output.push('\n');
        }
        output
    }
}

static MAP: LazyLock<Map> = LazyLock::new(|| Map::generate());

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    println!("Received request: {}", String::from_utf8_lossy(&buffer));
    stream.write_all(MAP.render().as_bytes())?;
    stream.flush()
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind TCP listener");
    println!("listening on 127.0.0.1:7878");
    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_contains_tree_pattern() {
        let rendered = MAP.render();

        assert!(rendered.contains('T'));
        assert_eq!(rendered.lines().count(), HEIGHT);
    }
}
