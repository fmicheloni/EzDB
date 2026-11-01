mod engine;
mod parser;
use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
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

fn handle_client(db_engine: &Mutex<DbEngine>, stream: TcpStream) -> std::io::Result<()> {
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
            Ok(cmd) => {
                let result = db_engine.lock().unwrap().execute_command(cmd);
                match result {
                    Ok(outcome) => DbResponse::Ok { ok: true, outcome },
                    Err(e) => DbResponse::Err {
                        ok: false,
                        error: format!("{:?}", e),
                    },
                }
            }
            Err(e) => DbResponse::Err {
                ok: false,
                error: format!("{:?}", e),
            },
        };
        writeln!(writer, "{}", serde_json::to_string(&response).unwrap())?;
    }
}

fn main() -> std::io::Result<()> {
    // without atomic reference counting we cannot share multiple mutable referenecs
    let db_engine = Arc::new(Mutex::new(DbEngine::new()));

    let listener = TcpListener::bind("127.0.0.1:6379")?;

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("failed to accept connection: {e}");
                continue;
            }
        };
        let db = Arc::clone(&db_engine);

        // TODO: very slow, use thread pool / async io
        thread::spawn(move || {
            if let Err(e) = handle_client(&db, stream) {
                eprintln!("client error: {e}");
            }
        });

        // if let Err(e) = stream.and_then(|s| handle_client(&db_engine, s)) {
        //     eprintln!("client error: {e}");
        // }
    }
    Ok(())
}
