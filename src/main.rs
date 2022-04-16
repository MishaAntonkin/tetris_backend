use std::thread::JoinHandle;

struct ThreadPool {
    id: usize,
    // sender send message to thread
    workers: Vec<Worker>
}

struct Worker {
    thread: Option<JoinHandle<()>>,

}

struct RoomsManager {
    // store room
    // passes to room websocket connection
    // handles join/ unjoin members
}


fn main() {
    println!("Hello, world!");
}
