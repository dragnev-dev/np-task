use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::{env, io, thread};

fn handle_client(stream: TcpStream, data: &HashMap<u32, String>) -> io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;

    loop {
        let mut request = String::new();
        reader.read_line(&mut request)?;
        let request = request.trim();
        print!("Request: {}, ", request);

        let response = match request.parse::<u32>() {
            Ok(key) => {
                match data.get(&key) {
                    Some(value) => {
                        println!("{}", value);
                        format!("{}: {}", key, value)
                    },
                    None => {
                        let result = String::from("no match");
                        println!("{}", result);
                        result
                    }
                }
            }
            Err(_) => {
                match data.iter().find(|(_, value)| value == &&request) {
                    Some((key, _)) => {
                        println!("{}", key);
                        format!("{}: {}", key, request)
                    },
                    None => {
                        let response = String::from("no match");
                        println!("{}", response);
                        response
                    }
                }
            }
        };

        writer.write_all(response.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()?;
    }
}

fn main() -> io::Result<()> {
    // Load data from CSV file
    let data_file = env::current_dir()?.join("assets/places_postcodes.csv");
    let mut data: HashMap<u32, String> = HashMap::new();

    let file = File::open(&data_file)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let key = parts[0].parse().unwrap_or(0);
            let value = String::from(parts[1]);
            data.insert(key, value);
        }
    }

    // Start TCP server
    let listener = TcpListener::bind("127.0.0.1:46420")?;
    println!("Server is running!");
    println!("Listening on 127.0.0.1:46420...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let data = data.clone();
                thread::spawn(move || {
                    if let Err(err) = handle_client(stream, &data) {
                        eprintln!("Error handling client: {}", err);
                    }
                });
            }
            Err(err) => {
                eprintln!("Failed to accept client: {}", err);
            }
        }
    }

    Ok(())
}
