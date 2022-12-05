use std::collections::HashMap;

struct Action {
    name: String,
    defeats: String,
    points: i32,
}

impl Action {
    fn new(name: &str, defeats: &str, points: i32) -> Action {
        Action {
            name: name.to_string(),
            defeats: defeats.to_string(),
            points,
        }
    }
}

fn main() {
    let mut actions: HashMap<String, Action> = HashMap::new();
    actions.insert("A".to_string(), Action::new("Rock", "Z", 1));
    actions.insert("B".to_string(), Action::new("Paper", "X", 2));
    actions.insert("C".to_string(), Action::new("Scissors", "Y", 3));
    actions.insert("X".to_string(), Action::new("Rock", "C", 1));
    actions.insert("Y".to_string(), Action::new("Paper", "A", 2));
    actions.insert("Z".to_string(), Action::new("Scissors", "B", 3));

    let mut res1: Vec<i32> = vec![];
    let mut res2: Vec<i32> = vec![];

    include_str!("../input.txt").lines().for_each(|line| {
        let r: Vec<&str> = line.split(' ').collect();
        res1.push(get_result_1(r[0], r[1], &actions));
        res2.push(get_result_2(r[0], r[1], &actions));
    });

    println!("Total score exc 1: {}", res1.iter().sum::<i32>());
    println!("Total score exc 2: {}", res2.iter().sum::<i32>());
}

fn get_result_1(a: &str, b: &str, m: &HashMap<String, Action>) -> i32 {
    let mut score: i32 = 0;
    if m[a].name == m[b].name {
        score = 3
    } else if m[b].defeats == a {
        score = 6
    }
    score + m[b].points
}

fn get_result_2(a: &str, b: &str, m: &HashMap<String, Action>) -> i32 {
    let mut score: i32 = 0;
    if b == "X" {
        score = m[&m[a].defeats].points
    } else if b == "Y" {
        score = m[a].points + 3
    } else {
        for (_k, v) in m {
            if v.defeats == a {
                score = v.points + 6;
                break;
            }
        }
    }
    return score;
}
