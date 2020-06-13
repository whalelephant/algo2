use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn input() -> Vec<u32> {
    let mut list = Vec::new();
    
    let path = Path::new("./week3/input.txt");
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}"),
        Ok(file) => file,
    };

    let reader = BufReader::new(&file);
    for oklines in reader.lines() {
        for value in oklines.unwrap().trim_end().split(' ') {
            list.push(value.parse::<u32>().unwrap());
        }
    }
    list
}

pub fn median() {
    let list = input();

    // Max Heap containing smallest i/2 elements in list
    let mut h_low = BinaryHeap::new();

    // Min Heap containing largest i/2 elements in the list
    // must use Reverse(x) for push as default is max
    // Reverse struct is tuple, access u32 in field 0
    let mut h_high = BinaryHeap::new();

    // list of medians for assignment (sum of medians mod 10000)
    let mut m = 0_f64;

    //  you should treat this as a stream of numbers, arriving one by one
    for v in list {
        // initialise
        if h_low.peek().is_none() {
            h_low.push(v);
            m = v as f64;
        } else {

            let max_low = h_low.peek().unwrap();
            if v < *max_low {
                h_low.push(v);
            } else {
                h_high.push(Reverse(v));
            }

            // adjust if not balanced
            if h_low.len()  ==  h_high.len() + 2 {
                let max_low = h_low.pop().unwrap();
                h_high.push(Reverse(max_low));
            } 
            
            if h_high.len()  == h_low.len() + 2 {
                let min_high = h_high.pop().unwrap();
                h_low.push(min_high.0);
            } 
            
            if h_low.len()  == h_high.len() + 1 {
                m = m + (*h_low.peek().unwrap() as f64);
            } else if h_high.len()  == h_low.len() + 1 {
                m = m + (h_high.peek().unwrap().0 as f64);
            } else if h_low.len() == h_high.len() {
                // this method is specified in the assignment, do not interpolate
                m = m + (*h_low.peek().unwrap() as f64);
            } else {
                panic!("tree is not balanced after balancing")
            }
        }
    }
    println!("sum of medians {:?}", m);
    println!("mod of sum of medians {:?}", m % 10000_f64);
}
