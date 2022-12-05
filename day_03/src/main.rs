use std::time::Instant;
fn main() {
    let start = Instant::now();
    // let alphabet = ('a'..='z').into_iter().collect::<Vec<char>>();

    let alphabet: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let input: Vec<&str> = include_str!("../input.txt").lines().collect();
    let mut result: Vec<i32> = vec![];

    // exc.1
    for line in input.iter() {
        let (c1, c2) = line.split_at(line.len() / 2);
        for c in c1.chars() {
            if c2.find(c).is_some() {
                result.push((alphabet.find(c).unwrap() as i32) + 1);
                break;
            };
        }
    }

    // let mut groups: Vec<Vec<&str>> = Vec::new(); //vec![vec!["abc"]];

    // exc.2
    let mut p: i32 = 0;

    for (i, _line) in input.iter().enumerate() {
        if (i + 1) % 3 == 0 && i != 0 {
            // groups.push(vec![input[i - 2], input[i - 1], input[i]]);
            for c in input[i].chars() {
                if input[i - 1].find(c).is_some() && input[i - 2].find(c).is_some() {
                    p += (alphabet.find(c).unwrap() as i32) + 1;
                    break;
                }
            }
        }
    }

    println!("Exc. 1 result: {:?}", result.iter().sum::<i32>());
    print!("Exc. 2 result: {}\n", p);
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
