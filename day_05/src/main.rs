use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    let lines: Vec<_> = contents.split("\n").collect();

    //find first empty line
    let split: usize = lines.iter().position(|f| f.is_empty()).unwrap() - 1;
    let cargo_input: Vec<_> = lines.iter().take(split).collect();
    let mut cargo: Vec<String> = vec![];

    //push chars in the correct cargo entry
    for (i, _line) in cargo_input.iter().enumerate() {
        for (n, c) in cargo_input[i].chars().enumerate() {
            if c.is_alphabetic() {
                // determine position it needs to go in
                let pos = (n - 1) / 4;
                // if the position does not exist, add enough entries to the vector
                if (cargo.len() as i32 - 1) < pos as i32 {
                    for _i in cargo.len()..=pos {
                        cargo.push("".to_string());
                    }
                }
                cargo[pos].push(c);
            }
        }
    }

    //reverse to get start position
    cargo = cargo.iter().map(|c| reverse_string(c)).collect();
    let moves: Vec<_> = lines.iter().filter(|l| l.starts_with('m')).collect();

    // duplicate start position for exc.2
    let mut cargo2 = cargo.clone();

    //execute moves exc.1
    moves.iter().for_each(|m| {
        let (crates, from, to) = get_crane_movement(m);
        for _i in 0..crates {
            let c = cargo[from].chars().last().unwrap();
            cargo[to].push(c);
            cargo[from].pop();
        }
    });

    print!("\nCrate location with 1st crane: ");
    cargo.iter().for_each(|line| {
        if !line.is_empty() {
            print!("{}", line.chars().last().unwrap())
        }
    });

    //execute moves exc.2
    moves.iter().for_each(|m| {
        let (crates, from, to) = get_crane_movement(m);
        let s = cargo2[from].clone();
        let (a, b) = s.split_at(s.len() - crates as usize);
        cargo2[from] = a.to_string();
        let n = String::from(&cargo2[to]) + b;
        cargo2[to] = n;
    });

    print!("\nCrate location with 2nd crane: ");
    cargo2.iter().for_each(|line| {
        if !line.is_empty() {
            print!("{}", line.chars().last().unwrap())
        }
    });

    let duration = start.elapsed();
    println!("\n\nTime elapsed is: {:?}", duration);
}

fn reverse_string(s: &String) -> String {
    return s.chars().rev().collect();
}

fn get_crane_movement(s: &str) -> (i32, usize, usize) {
    let x: Vec<i32> = s.split(' ').filter_map(|f| f.parse().ok()).collect();
    return (x[0], (x[1] - 1) as usize, (x[2] - 1) as usize);
}
