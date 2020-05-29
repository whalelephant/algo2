mod scc;
use scc::scc;
use std::thread;

fn main() {
    let child = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
        return scc();
    }).unwrap();
    
    let v = child.join().unwrap();
    println!("v: {:?}", v);
   
}
