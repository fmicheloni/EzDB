mod engine;
mod parser;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use serde::Serialize;

use crate::engine::{DbEngine, Outcome};

#[derive(Serialize)]
#[serde(untagged)]
enum DbResponse {
    Ok {
        ok: bool,
        #[serde(flatten)]
        outcome: Outcome,
    },
    Err {
        ok: bool,
        error: String,
    },
}

fn handle_client(db_engine: &mut DbEngine, stream: TcpStream) -> std::io::Result<()> {
    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();
        let n = reader.read_line(&mut line)?;

        if n == 0 {
            // client closed the connection
            return Ok(());
        }
        println!("got: {}", line.trim_end());
        let response = match parser::parse(&line) {
            Ok(cmd) => match db_engine.execute_command(cmd) {
                Ok(outcome) => DbResponse::Ok { ok: true, outcome },
                Err(e) => DbResponse::Err {
                    ok: false,
                    error: format!("{:?}", e),
                },
            },
            Err(e) => DbResponse::Err {
                ok: false,
                error: format!("{:?}", e),
            },
        };
        writeln!(writer, "{}", serde_json::to_string(&response).unwrap())?;
    }
}

fn main() -> std::io::Result<()> {
    let mut db_engine = DbEngine::new();
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    for stream in listener.incoming() {
        if let Err(e) = stream.and_then(|s| handle_client(&mut db_engine, s)) {
            eprintln!("client error: {e}");
        }
    }
    Ok(())
}
