use std::collections::HashMap;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use lazy_static::lazy_static;

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


// fn main() -> io::Result<()>{
//     // load the zips
//     let mut zips_serialized =
//         Reader::from_path("assets/places_postcodes.csv")?;
//
//     zips_serialized.set_headers(csv::StringRecord::from(vec!["zip", "place", "municipality", "area"]));
//
//     // let mut map = BidiMap::new();
//     let mut data = HashMap::new();
//     data.insert("hello".to_string(), "world".to_string());
//     data.insert("foo".to_string(), "bar".to_string());
//     data.insert("rust".to_string(), "lang".to_string());
//
//     // for result in zips_serialized.deserialize() {
//     //     // We must tell Serde what type we want to deserialize into.
//     //     let record: ZipEntry = result?;
//     //     // println!("{}, {}", record.zip, record.place);
//     //     // unsafe { ZIPS_TOWNS.put(record.zip, record.place); }
//     //     // https://github.com/rust-lang-nursery/lazy-static.rs#example
//     //     // HASHMAP_A.__private_field..insert(record.zip, &record.place);
//     //     map.put(record.zip, record.place);
//     // }
//
//     // Enable port 7878 binding
//     // let receiver_listener = TcpListener::bind("127.0.0.1:7878").expect("Failed and bind with the sender");
//     let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
//
//     // Getting a handle of the underlying thread.
//     // let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
//     // // listen to incoming connections messages and bind them to a sever socket address.
//     // for stream in receiver_listener.incoming() {
//     //     let stream = stream.expect("failed");
//     //     // let the receiver connect with the sender
//     //     let handle = thread::spawn(move || unsafe {
//     //         //receiver failed to read from the stream
//     //         handle_client(stream, &map).unwrap_or_else(|error| eprintln!("{:?}", error))
//     //     });
//     //
//     //     // Push messages in the order they are sent
//     //     thread_vec.push(handle);
//     // }
//     //
//     // for handle in thread_vec {
//     //     // return each single value Output contained in the heap
//     //     handle.join().unwrap();
//     // }
//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 // Spawn a new thread to handle the client
//                 let data_ref = &data;
//                 thread::spawn(move || {
//                     handle_client(stream, data_ref);
//                 });
//             }
//             Err(e) => {
//                 eprintln!("error accepting client: {}", e);
//             }
//         }
//     }
//     // success value
//     Ok(())
// }

struct BidiMap<A, B> {
    left_to_right: HashMap<Rc<A>, Rc<B>>,
    right_to_left: HashMap<Rc<B>, Rc<A>>,
}

impl<A, B> BidiMap<A, B>
    where
        A: Eq + Hash,
        B: Eq + Hash,
{
    fn new() -> Self {
        BidiMap {
            left_to_right: HashMap::new(),
            right_to_left: HashMap::new(),
        }
    }

    fn put(&mut self, a: A, b: B) {
        let a = Rc::new(a);
        let b = Rc::new(b);
        self.left_to_right.insert(a.clone(), b.clone());
        self.right_to_left.insert(b, a);
    }

    fn get(&self, a: &A) -> Option<&B> {
        self.left_to_right.get(a).map(Deref::deref)
    }

    fn get_reverse(&self, b: &B) -> Option<&A> {
        self.right_to_left.get(b).map(Deref::deref)
    }
}

lazy_static! {
    static ref HASHMAP_A: HashMap<u16, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.remove(&0);
        m
    };

    static ref HASHMAP_B: HashMap<&'static str, u16> = {
        let mut m = HashMap::new();
        m.insert("0", 0);
        m.remove(&"0");
        m
    };
}

// #[derive(Debug, Deserialize)]
// struct ZipEntry {
//     zip: u16,
//     place: String
// }

// Handle access stream
//create a struct to hold the stream’s state
// Perform I/O operations
// fn handle_client(mut stream: TcpStream, data: &BidiMap<u16, String>) -> io::Result<()> {
//     let mut buffer = [0; 1024];
//     let size = stream.read(&mut buffer)?;
//     let request = String::from_utf8_lossy(&buffer[..size]).trim().to_string();
//     println!("Request : {}", request);
//     let key = request.parse::<u16>().unwrap_or_default();
//     let response = data
//         .get(&key)
//         .map_or_else(
//             || data.get_reverse(&request).map_or("no match".to_string(), |v| v.to_string()),
//             |v| v.to_string(),
//         );
//     println!("Response: {}", response);
//     stream.write(response.as_bytes())?;
//     // Ok(())
// }


// fn handle_client(stream: TcpStream, data: Rc<HashMap<String, String>>) {
//     let mut reader = BufReader::new(&stream);
//     let mut writer = stream.try_clone().unwrap();
//
//     loop {
//         let mut request = String::new();
//
//         // Read a line from the client
//         match reader.read_line(&mut request) {
//             Ok(0) => return, // Connection closed
//             Ok(_) => {
//                 // Trim whitespace and check if the key exists in the data structure
//                 let key = request.trim();
//                 let response = match data.get(key) {
//                     Some(value) => value.to_string(),
//                     None => "no match".to_string(),
//                 };
//
//                 // Send the response to the client
//                 writer.write(response.as_bytes()).unwrap();
//                 writer.write(b"\n").unwrap();
//             }
//             Err(_) => return, // Error reading from client
//         }
//     }
// }


// fn two_way_lookup(request: Cow<str>) -> String {
//     let is_number = request.parse::<u16>();
//
//     let dir_a_lookup = ZIPS_TOWNS.get(&is_number.unwrap());
//     let dir_b_lookup = ZIPS_TOWNS.get_reverse(&request.to_string());
//
//     if dir_a_lookup != None { return String::from(dir_a_lookup.unwrap()) }
//     if dir_b_lookup != None { return dir_b_lookup.unwrap().to_string() }
//     return String::from("¯\\_(ツ)_/¯");
// }
