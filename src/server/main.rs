use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::{env, io, thread};

fn handle_client(stream: TcpStream, data: &HashMap<u32, String>) {
    let mut reader = BufReader::new(&stream);
    let mut writer = stream.try_clone().unwrap();

    loop {
        let mut request = String::new();

        // Read a line from the client
        match reader.read_line(&mut request) {
            Ok(0) => return, // Connection closed
            Ok(_) => {
                // Trim whitespace and check if the key exists in the data structure
                let request = request.trim();
                println!("Request : {}", request);
                let response = match request.parse::<u32>() {
                    Ok(key) => {
                        match data.get(&key) {
                            Some(value) => format!("{}: {}", key, value),
                            None => String::from("no match"),
                        }
                    }
                    Err(_) => {
                        match data.iter().find(|(_, value)| value == &&request) {
                            Some((key, _)) => format!("{}: {}", key, request),
                            None => String::from("no match"),
                        }
                    }
                };

                println!("Response: {}", response);
                // Send the response to the client
                writer.write(response.as_bytes()).unwrap();
                writer.write(b"\n").unwrap();
            }
            Err(_) => return, // Error reading from client
        }
    }
}

fn main() -> io::Result<()> {
    // Load data from CSV file
    let data_file = env::current_dir()?.join("assets/data.csv");
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
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let data = data.clone();
                // Spawn a new thread to handle the client
                thread::spawn(move || {
                    handle_client(stream, &data);
                });
            }
            Err(e) => {
                eprintln!("error accepting client: {}", e);
            }
        }
    }

    Ok(())
}
