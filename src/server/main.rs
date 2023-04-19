use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(stream: TcpStream, data: Arc<Mutex<HashMap<String, String>>>) {
    let mut reader = BufReader::new(&stream);
    let mut writer = stream.try_clone().unwrap();

    loop {
        let mut request = String::new();

        // Read a line from the client
        match reader.read_line(&mut request) {
            Ok(0) => return, // Connection closed
            Ok(_) => {
                // Trim whitespace and check if the key exists in the data structure
                let key = request.trim();
                println!("Request : {}", request);
                let response = match data.lock().unwrap().get(key) {
                    Some(value) => value.to_string(),
                    None => "no match".to_string(),
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

fn main() {
    // Create a TCP listener on port 8080
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    // Create a hashmap to store the key-value pairs
    let mut data = HashMap::new();
    data.insert("hello".to_string(), "world".to_string());
    data.insert("foo".to_string(), "bar".to_string());
    data.insert("rust".to_string(), "lang".to_string());

    // Wrap the data hashmap in an Arc and Mutex so it can be shared and synchronized between threads
    let data_arc = Arc::new(Mutex::new(data));

    // Accept incoming client connections
    for stream in listener.incoming() {
        println!("55");
        match stream {
            Ok(stream) => {
                // Clone the Arc to pass a reference to the shared data to the new thread
                let data_arc_clone = data_arc.clone();
                // Spawn a new thread to handle the client
                thread::spawn(move || {
                    handle_client(stream, data_arc_clone);
                });
            }
            Err(e) => {
                eprintln!("error accepting client: {}", e);
            }
        }
    }
}
