use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

// find all the shortest distance starting from node 1 at index 0
// not going to use heap as none available with delete + bubble api
// assuming graph is connected, directed, non-negative lengths
pub fn shortest_paths() {
    let adj_list = input();
    // initialise with source in x - visited nodes and the shortest distance
    // initialise with the shortest distance of 0 to 0 as Some(0)
    let mut visited = HashMap::new();
    visited.insert(1, 0);

    while adj_list.len() > visited.keys().len() {
        println!("visited length: {}", visited.keys().len());
        
        let mut min_greedy_score = 1000000;
        let mut min_vertex = 0;
        // for every tail in the visited portion of the graph
        for tail in visited.keys() {
            // we look for the edge crossing into the unvisited portion
            for head in &adj_list[*tail as usize-1] {
                // if already visited, then it does not cross the boundary 
                if visited.contains_key(&head[0]) {
                    continue;
                } else {
                    let tail_score = visited.get(tail).unwrap();
                    let greedy_score = tail_score + head[1];
                    if greedy_score < min_greedy_score {
                        min_greedy_score = greedy_score;
                        min_vertex = head[0];
                    }
                }
            }
        }
        visited.insert(min_vertex, min_greedy_score);
    }

    // for assignment - 7,37,59,82,99,115,133,165,188,197
    println!("{},{},{},{},{},{},{},{},{},{}",
        visited.get(&7).unwrap(),
        visited.get(&37).unwrap(),
        visited.get(&59).unwrap(),
        visited.get(&82).unwrap(),
        visited.get(&99).unwrap(),
        visited.get(&115).unwrap(),
        visited.get(&133).unwrap(),
        visited.get(&165).unwrap(),
        visited.get(&188).unwrap(),
        visited.get(&197).unwrap(),
    );
}


// input format:
// tail_vertex_number head_vertex_number,weight head_vertex_number,weight
// output format:
// [
//     [
//         [head, weight], [head, weight]
//     ]
// ]
pub fn input() -> Vec<Vec<Vec<u32>>> {
    let mut adj_list = Vec::new();
    
    let path = Path::new("./week2/input.txt");
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}"),
        Ok(file) => file,
    };

    let reader = BufReader::new(&file);
    for oklines in reader.lines() {
        let mut node_list = Vec::new();
        for value in oklines.unwrap().trim_end().split(' ') {
            let mut head = Vec::new();
            for v in value.split(',') {
                println!("v {}", v);    
                head.push(v.parse::<u32>().unwrap());
            }
            node_list.push(head);
        }
        if node_list.len() > 1 {
            adj_list.push(node_list.split_off(1));
        } else {
            adj_list.push(vec![]);
        }
    }
    adj_list
}