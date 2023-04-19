use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() {
    // Connect to the server
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:46420") {
        println!("Connected to the server");
        println!("Enter a request (or 'quit' to exit):");

        loop {
            // Read input from the user
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let key = input.trim();

            if key == "quit" {
                // Exit loop and close connection
                break;
            }

            // Send request to the server
            stream.write(key.as_bytes()).unwrap();
            stream.write(b"\n").unwrap();

            // Read response from the server
            let mut response = String::new();
            let mut reader = BufReader::new(&stream);
            reader.read_line(&mut response).unwrap();
            println!("{}", response);
        }
    } else {
        println!("Failed to connect to server.");
    }
}
