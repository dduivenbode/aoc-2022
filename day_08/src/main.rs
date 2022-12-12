use std::time::Instant;

fn main() {
    let start = Instant::now();
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();
    //create vector of vectors for easier comparison
    let mut grid: Vec<Vec<u32>> = vec![];
    for (i, l) in lines.iter().enumerate() {
        if grid.len() <= i {
            grid.push(vec![])
        }
        l.chars()
            .for_each(|c| grid[i].push(c.to_digit(10).unwrap()))
    }
    let mut visible_trees = 0;

    //get visible trees
    for (y, line) in grid.iter().enumerate() {
        for (x, height) in grid[y].iter().enumerate() {
            if is_visible(x, y, height.clone(), &line, &grid) {
                visible_trees += 1;
            }
        }
    }
    let mut max_score = 0;
    //get max scenic score
    for (y, line) in grid.iter().enumerate() {
        for (x, height) in grid[y].iter().enumerate() {
            let score = get_score(x, y, height.clone(), &line, &grid);
            if score > max_score {
                max_score = score;
            }
        }
    }

    // println!("{}", lines[2].chars().nth(2).unwrap());
    println!("{}", visible_trees);
    println!("{}", max_score);

    let duration = start.elapsed();
    println!("\n\nTime elapsed is: {:?}", duration);
}

fn get_score(x: usize, y: usize, height: u32, line: &Vec<u32>, grid: &Vec<Vec<u32>>) -> i32 {
    if x == 0 || y == 0 || x == (line.len() - 1) || y == (grid.len() - 1) {
        0
    } else {
        get_range(x, y, height, &line, &grid, "left", &mut 0)
            * get_range(x, y, height, &line, &grid, "right", &mut 0)
            * get_range(x, y, height, &line, &grid, "up", &mut 0)
            * get_range(x, y, height, &line, &grid, "down", &mut 0)
    }
}

fn get_range(
    x: usize,
    y: usize,
    height: u32,
    line: &Vec<u32>,
    grid: &Vec<Vec<u32>>,
    direction: &str,
    score: &mut i32,
) -> i32 {
    *score += 1;
    match direction {
        "left" => {
            if grid[y][x - 1] >= height || x - 1 == 0 {
                *score
            } else {
                get_range(x - 1, y, height, line, grid, direction, score)
            }
        }
        "right" => {
            if grid[y][x + 1] >= height || x + 1 == line.len() - 1 {
                *score
            } else {
                get_range(x + 1, y, height, line, grid, direction, score)
            }
        }
        "up" => {
            if grid[y - 1][x] >= height || y - 1 == 0 {
                *score
            } else {
                get_range(x, y - 1, height, line, grid, direction, score)
            }
        }
        "down" => {
            if grid[y + 1][x] >= height || y + 1 == grid.len() - 1 {
                *score
            } else {
                get_range(x, y + 1, height, line, grid, direction, score)
            }
        }
        &_ => 0,
    }
}

fn is_visible(x: usize, y: usize, height: u32, line: &Vec<u32>, grid: &Vec<Vec<u32>>) -> bool {
    if x == 0 || y == 0 || x == (line.len() - 1) || y == (grid.len() - 1) {
        true
    } else {
        return check_visibility(x, y, height, &line, &grid, "left")
            || check_visibility(x, y, height, &line, &grid, "right")
            || check_visibility(x, y, height, &line, &grid, "up")
            || check_visibility(x, y, height, &line, &grid, "down");
    }
}

fn check_visibility(
    x: usize,
    y: usize,
    height: u32,
    line: &Vec<u32>,
    grid: &Vec<Vec<u32>>,
    direction: &str,
) -> bool {
    match direction {
        "left" => {
            if grid[y][x - 1] < height {
                if x - 1 == 0 {
                    true
                } else {
                    check_visibility(x - 1, y, height, line, grid, direction)
                }
            } else {
                false
            }
        }
        "right" => {
            if grid[y][x + 1] < height {
                if x + 1 == line.len() - 1 {
                    true
                } else {
                    check_visibility(x + 1, y, height, line, grid, direction)
                }
            } else {
                false
            }
        }
        "up" => {
            if grid[y - 1][x] < height {
                if y - 1 == 0 {
                    true
                } else {
                    check_visibility(x, y - 1, height, line, grid, direction)
                }
            } else {
                false
            }
        }
        "down" => {
            if grid[y + 1][x] < height {
                if y + 1 == grid.len() - 1 {
                    true
                } else {
                    check_visibility(x, y + 1, height, line, grid, direction)
                }
            } else {
                false
            }
        }
        &_ => false,
    }
}
