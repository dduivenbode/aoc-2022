use std::collections::HashSet;
// use std::fs;
use std::time::Instant;
fn main() {
    let start = Instant::now();

    let lines: Vec<(&str, i32)> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (direction, x) = line.split_at(1);
            (direction, x.trim().parse().ok().unwrap())
        })
        .collect();

    println!(
        "\nExc 1: tail went to {} unique locations",
        move_head(&lines, 2)
    );
    println!(
        "Exc 2: tail went to {} unique locations",
        move_head(&lines, 10)
    );
    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
}

fn move_head(lines: &Vec<(&str, i32)>, knots: i32) -> i32 {
    let mut tails: Vec<(i32, i32)> = Vec::new();
    let mut tail_pos: HashSet<(i32, i32)> = HashSet::new();
    tail_pos.insert((0, 0));
    for _i in 0..knots {
        tails.push((0, 0))
    }

    for line in lines.iter() {
        for _i in 0..line.1 {
            match line.0 {
                "R" => {
                    tails[0].0 += 1;
                }
                "L" => {
                    tails[0].0 -= 1;
                }
                "U" => {
                    tails[0].1 += 1;
                }
                "D" => {
                    tails[0].1 -= 1;
                }
                &_ => {}
            }

            for i in 0..tails.len() - 1 {
                if !tail_is_near(&tails[i], &tails[i + 1]) {
                    tails[i + 1] = move_tail(&tails[i], &tails[i + 1]);
                    //final tail?
                    if i == tails.len() - 2 {
                        tail_pos.insert(tails[i + 1]);
                    }
                }
            }
        }
    }
    tail_pos.len() as i32
}

fn tail_is_near(h: &(i32, i32), t: &(i32, i32)) -> bool {
    let x = h.0 - 1..=h.0 + 1;
    let y = h.1 - 1..=h.1 + 1;
    x.contains(&t.0) && y.contains(&t.1)
}

fn move_tail(h: &(i32, i32), t: &(i32, i32)) -> (i32, i32) {
    let mut nt = t.clone();
    if nt.0 < h.0 {
        nt.0 += 1;
    } else if nt.0 > h.0 {
        nt.0 -= 1;
    }
    if nt.1 < h.1 {
        nt.1 += 1;
    } else if nt.1 > h.1 {
        nt.1 -= 1;
    }
    nt
}
