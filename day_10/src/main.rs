use std::time::Instant;
fn main() {
    let start = Instant::now();
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();

    let mut x = 1;
    let mut cycle = 0;
    let mut pixel = 0;
    let mut signals: Vec<i32> = Vec::new();

    for (_i, line) in lines.iter().enumerate() {
        execute_cycle(&mut cycle, &mut signals, &mut pixel, x);
        match line.split_once(' ') {
            Some((_x, v)) => {
                execute_cycle(&mut cycle, &mut signals, &mut pixel, x);
                x += v.trim().parse::<i32>().ok().unwrap();
            }
            None => {}
        }
    }
    println!(
        "\n\nCombined signal strength is: {}",
        signals.iter().sum::<i32>()
    );

    let duration = start.elapsed();
    println!("\nTime elapsed is: {:?}", duration);
}

fn execute_cycle<'c, 's, 'p>(c: &'c mut i32, s: &'s mut Vec<i32>, p: &'p mut i32, x: i32) {
    *c += 1;
    print_pixel(p, x);
    if *c % 40 == 20 && *c <= 220 {
        //calculate strength
        s.push(*c * x);
    }
}

fn print_pixel<'p>(pixel: &'p mut i32, x: i32) {
    if *pixel == 0 {
        print!("\n")
    }
    if (x - 1..=x + 1).contains(pixel) {
        print!("#")
    } else {
        print!(".")
    }
    *pixel += 1;
    if *pixel == 40 {
        *pixel = 0;
    }
}
