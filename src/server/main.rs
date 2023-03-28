use std::io;
use std::time;
use std ::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::thread;
use csv::Reader;
use serde::Deserialize;
use std::collections::HashMap;
use std::rc::Rc;
use std::hash::Hash;
use std::ops::Deref;

fn main() -> io::Result<()>{
    println!("10");
    let mut zips_serialized =
        Reader::from_path("assets/places_postcodes.csv")?;
    println!("16");

    zips_serialized.set_headers(csv::StringRecord::from(vec!["zip", "place", "municipality", "area"]));
    // let mut zips: Vec<ZipEntry> = Vec::new();
    let mut zipsTowns = BidiMap::new();

    for result in zips_serialized.deserialize() {
        // We must tell Serde what type we want to deserialize into.
        let record: ZipEntry = result?;
        // zips.push(record);
        println!("{}, {}", record.zip, record.place);

        zipsTowns.put(record.zip, record.place);
    }
    println!("zips loaded...");
    println!("{:?}", zipsTowns.get(&1000).unwrap());
    println!("{:?}", zipsTowns.get_reverse(&"София".to_string()).unwrap());

    let dirALookup = zipsTowns.get(&65535);
    let dirBLookup = zipsTowns.get_reverse(&"Чичово".to_string());

    if dirALookup == None && dirBLookup == None {
        println!("¯\\_(ツ)_/¯")
    } else {
        if dirALookup != None { println!("{}", dirALookup.unwrap()) }
        if dirBLookup != None { println!("{}", dirBLookup.unwrap()) }
    }


    // Enable port 7878 binding
    let receiver_listener = TcpListener::bind("127.0.0.1:7878").expect("Failed and bind with the sender");
    // Getting a handle of the underlying thread.
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    // listen to incoming connections messages and bind them to a sever socket address.
    for stream in receiver_listener.incoming() {
        let stream = stream.expect("failed");
        // let the receiver connect with the sender
        let handle = thread::spawn(move || {
            //receiver failed to read from the stream
            handle_sender(stream).unwrap_or_else(|error| eprintln!("{:?}",error))
        });

        // Push messages in the order they are sent
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        // return each single value Output contained in the heap
        handle.join().unwrap();
    }
    // success value
    Ok(())
}

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

#[derive(Debug, Deserialize)]
struct ZipEntry {
    zip: u16,
    place: String,
    // municipality: String,
    // area: String
}

// Handle access stream
//create a struct to hold the stream’s state
// Perform I/O operations
fn handle_sender(mut stream: TcpStream) -> io::Result<()>{
    // Handle multiple access stream
    let mut buf = [0;512];
    for _ in 0..1000{
        // let the receiver get a message from a sender
        let bytes_read = stream.read(&mut buf)?;
        // sender stream in a mutable variable
        if bytes_read == 0{
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
        // Print acceptance message
        //read, print the message sent
        println!("from the sender:{}",String::from_utf8_lossy(&buf));
        // And you can sleep this connection with the connected sender
        thread::sleep(time::Duration::from_secs(1));
    }
    // success value
    Ok(())
}
