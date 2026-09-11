use project_rust_kvstore::Store;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream, store: &mut Store) -> std::io::Result<()> {
    let mut reader = BufReader::new(&stream);
    let mut writer = &stream;

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let parts: Vec<&str> = line.trim().splitn(3, ' ').collect();

    match parts.as_slice() {
        ["SET", key, value] => {
            store.set(key.to_string(), value.to_string())?;
            writeln!(writer, "OK")?;
        }
        ["GET", key] => match store.get(key) {
            Some(value) => writeln!(writer, "{}", value)?,
            None => writeln!(writer, "NOT_FOUND")?,
        },
        ["DELETE", key] => {
            store.delete(key)?;
            writeln!(writer, "OK")?;
        }
        _ => {
            writeln!(writer, "ERROR: unknown command")?;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut store = Store::load("data.log")?;

    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream?;
        handle_client(stream, &mut store)?;
    }

    Ok(())
}