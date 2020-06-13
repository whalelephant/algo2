mod scc;
mod dijkstras;
mod maintain_median;
use scc::scc;
use std::thread;


fn main() {

    // week1
    let child = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
        return scc();
    }).unwrap();
    
    let v = child.join().unwrap();
    println!("v: {:?}", v);

    // week2
    dijkstras::shortest_paths();

    // week3
    maintain_median::median();
}
