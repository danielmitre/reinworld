use std::sync::LazyLock;

use futures_util::StreamExt;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, body::MessageBody, web};
use actix_ws::Message;

mod msg;

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

async fn ws_map(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    let _ = session.pong(&bytes).await;
                }
                Message::Binary(bytes) => {
                    match msg::Msg::from(bytes.to_vec()) {
                        msg::Msg::GetMap => {
                            let _ = session.text(MAP.render()).await;
                        }
                        msg::Msg::Move(dir) => {
                            eprintln!("move {dir:?}")
                        }
                        msg::Msg::Unknown => {
                            eprintln!("Unknown message type, skipping");
                        }
                    }
                }
                Message::Close(reason) => {
                    let _ = session.close(reason).await;
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("listening on http://127.0.0.1:8080/ws");

    HttpServer::new(|| App::new().route("/ws", web::get().to(ws_map)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
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
