use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    // get input in vector
    let exec = get_input(&contents);

    let mut dirs: HashMap<String, i32> = HashMap::new();
    let mut path: Vec<&str> = vec!["/"]; //used to keep track of what directory we are in

    for (_i, e) in exec.iter().enumerate() {
        if e.starts_with("$ cd") {
            //what directory are we changing to?
            let (_z, target) = e.split_at(5);
            //change content of 'path' vector to reflect current directory
            match target {
                "/" => path.truncate(1),               //keep the first
                ".." => path.truncate(path.len() - 1), //keep everything but the last
                _ => path.push(target),
            }
        } else if !e.starts_with("$") {
            //this line must be the result of an 'list' command
            //if the first part is a number, store it in the appropriate directories
            let res: Vec<_> = e.split(' ').collect();
            //if the parse returns OK, it's a number
            match res[0].parse::<i32>() {
                Ok(size) => {
                    for (i, _p) in path.iter().enumerate() {
                        let r = get_path(&path, i);
                        //if directory is already present in map, add size to the value
                        if dirs.contains_key(&r.clone()) {
                            //get a mutable value and add size to it in place
                            *dirs.get_mut(&r.clone()).unwrap() += size;
                        } else {
                            dirs.insert(r.clone().to_string(), size);
                        }
                    }
                }
                Err(_e) => {
                    // these lines are not relevant
                }
            }
        }
    }

    // get all directories small enough and sum the result
    let r: i32 = dirs
        .iter()
        .filter(|&(_k, v)| *v <= 100000)
        .map(|(_k, v)| v)
        .sum();

    println!("\nPart 1 result: {}", r);

    //part 2
    let space_needed = 30000000 - (70000000 - dirs.get("/").unwrap());

    // get all directories large enough, and get the smallest of the remaining entries
    let r2 = dirs
        .iter()
        .filter(|&(_k, v)| *v >= space_needed)
        .map(|(_k, v)| v)
        .min()
        .unwrap();

    println!("Part 2 result: {}", r2);

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
}

fn get_input(inp: &String) -> Vec<String> {
    inp.split("\n").map(|l| l.to_string()).collect()
}

fn get_path(v: &Vec<&str>, i: usize) -> String {
    if i == 0 {
        "/".to_string()
    } else if i == 1 {
        format!("/{}", v[i])
    } else {
        let mut p = String::new();
        for (n, x) in v.iter().enumerate() {
            if n == 0 {
                p = "/".to_string();
            } else if n == 1 {
                p = format!("/{}", x);
            } else {
                p = format!("{}/{}", p, x)
            }
            if n == i {
                break;
            }
        }
        p.to_string()
    }
}
