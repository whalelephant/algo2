use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn scc() -> Vec<u32> {
    let input = scc_input();
    println!("got input");

    // fin_times is map key = finishing_time, value = n
    let mut fin_times = HashMap::new();
    // leaders is node and leader count, slightly different to instructions
    let mut leaders = HashMap::new();
    
    // first pass, with graph g_rev
    dfs_loop(&input, true, &mut fin_times, &mut leaders);
    
    // second pass, with graph g, finishing_time
    dfs_loop(&input, false, &mut fin_times, &mut leaders);

    let mut scc_sizes = Vec::new();
    for v in leaders.values(){
        scc_sizes.push(*v);
    }

    // this is O(nlogn) sort
    scc_sizes.sort();
    scc_sizes
}

fn dfs_loop(
    input: &Vec<Vec<u32>>,
    first_pass: bool,
    fin_times: &mut HashMap<u32, u32>,
    leaders: &mut HashMap<u32, u32>,
) {
    let mut unexplored = HashMap::<u32, bool>::new();
    let g = map(input, first_pass, &mut unexplored);
    println!("done with map for first_pass = {}", first_pass);

    let mut time = 0;
    if first_pass {
        for i in g.keys() {
            if unexplored[&i] {
                dfs(first_pass, &g, *i, fin_times, leaders, &mut unexplored, &mut 0, &mut time);
            }
        }
    } else {
        let keylen = fin_times.keys().len() as u32;
        for i in (1..=keylen).rev() {
            let key = fin_times[&i];
            let mut leader = key;
            if unexplored[&key] {
                dfs(first_pass, &g, key, fin_times, leaders, &mut unexplored, &mut leader, &mut time);
            }
        }
    }
}

fn dfs(
    first_pass: bool,
    g: &HashMap<u32, Vec<u32>>,
    i: u32,
    fin_times: &mut HashMap<u32, u32>,
    leaders: &mut HashMap<u32, u32>,
    unexplored: &mut HashMap<u32, bool>,
    leader: &mut u32,
    time: &mut u32,
) {
    if let Some(is_unexplored) = unexplored.get_mut(&i) {
        *is_unexplored = false;
    } else {
        panic!("should have value");
    }

    if !first_pass {
        if let Some(l) = leaders.get_mut(leader) {
            *l = *l + 1;
        } else {
            leaders.insert(*leader, 1);
        }
    }

    if let Some(d) = g.get(&i) {
        for v in d {
            if unexplored[v] {
                dfs(first_pass, g, *v, fin_times, leaders, unexplored, leader, time);
            }
        }
    }
    if first_pass {
        *time = *time + 1;
        fin_times.insert(*time, i);
    }
}

// Utilities
fn scc_input() -> Vec<Vec<u32>> {
    let path = Path::new("./week1/input.txt");
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}"),
        Ok(file) => file,
    };
    let mut input_vec = Vec::new();
    let reader = BufReader::new(&file);
    for oklines in reader.lines() {
        let mut v = Vec::new();
        for value in oklines.unwrap().split(' ') {
            v.push(value.parse::<u32>().unwrap());
        }
        input_vec.push(v);
    }
    input_vec
}

fn map(input: &Vec<Vec<u32>>, first_pass: bool, unexplored: &mut HashMap<u32, bool>) -> HashMap<u32, Vec<u32>> {
    let mut full_map = HashMap::<u32, Vec<u32>>::new();
    for e in input {
        let mut h = e[0];
        let mut t = e[1];
        if first_pass {
            h = e[1];
            t = e[0];
        }
        if let Some(n) = full_map.get_mut(&h) {
            n.push(t);
        } else {
            full_map.insert(h, vec![t]);
        }
        // it can be either or both 
        unexplored.insert(e[0], true);
        unexplored.insert(e[1], true);
    }
    full_map
}
