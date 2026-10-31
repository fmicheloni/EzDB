mod engine;
mod parser;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use crate::engine::DbEngine;

fn main() -> std::io::Result<()> {
    let mut db_engine = DbEngine::new();
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    for stream in listener.incoming() {
        let stream = stream?;
        let mut writer = stream.try_clone()?;
        let mut reader = BufReader::new(stream);
        let mut line = String::new();

        loop {
            line.clear();
            let n = reader.read_line(&mut line)?;
            if n == 0 {
                break; // client closed the connection
            }
            println!("got: {}", line.trim_end());
            let response = match parser::parse(&line) {
                Ok(cmd) => db_engine.execute_command(cmd),
                Err(e) => Some(format!("ERROR: {:?}", e)),
            };
            writeln!(
                writer,
                "{}",
                response.unwrap_or("Something went wrong...".to_string())
            )?;
        }
    }
    Ok(())
}
