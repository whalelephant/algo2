use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;


pub fn total_hashmap() {
    let arr = input();
    let mut map: HashMap<i64, Vec<usize>> = HashMap::new();
    let mut total = 0;

    // insert into a HashMap<i64, Vec<usize>>
    // value map to the index of the input array
    for (i, v) in arr.iter().enumerate() {
        if let Some(a) = map.get_mut(v) {
            a.push(i);
        } else {
            map.insert(*v, vec![i]);
        }
    }

    for t in -10000_i64..=10000_i64 {
        for (x, v) in map.iter() {
            let y = t-x;
            if  *x != y {
                if let Some (y_arr) =  map.get(&y) {
                    total = total + ( v.len() * y_arr.len() );
                }
            }
        }
    }

    println!("total: {}", total/2);

}

pub fn total_sorted_arr() {
    let mut arr = input();
    let mut total = HashMap::new();
    arr.sort(); // O(nlogn)
    let mut i = 0;
    let mut j = arr.len()-1;
    println!("arr length: {}", j+1);

    while i < j {
        let mut sum = arr[i] + arr[j];
        if sum < -10000_i64 {
            i += 1;
        } else if sum > 10000_i64 {
            j = j-1;
        } else {
            for ii in i..j {
                let sum = arr[ii] + arr[j];
                if sum > 10000 {
                    break;
                } else if arr[ii] != arr[j] {
                    total.insert(sum, (ii, j));
                }
            }
            j = j - 1;
        }
    }
    println!("{}", total.keys().len());

}


// util
fn input() -> Vec<i64> {
    let mut list = Vec::new();

    let path = Path::new("./week4/input.txt");
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}"),
        Ok(file) => file,
    };

    let reader = BufReader::new(&file);
    for oklines in reader.lines() {
        for value in oklines.unwrap().trim_end().split(' ') {
            list.push(value.parse::<i64>().unwrap());
        }
    }
    list
}