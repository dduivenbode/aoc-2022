use std::time::Instant;
fn main() {
    let start = Instant::now();

    let sections: Vec<_> = include_str!("../input.txt")
        .lines()
        .into_iter()
        .map(|line| {
            let sections: Vec<&str> = line.split(',').collect();

            let s1 = get_section_list(&sections[0]);
            let s2 = get_section_list(&sections[1]);
            return [s1, s2];
        })
        .collect();

    let exc1: Vec<_> = sections
        .iter()
        .filter(|s| {
            if (s[0][0] >= s[1][0] && s[0][1] <= s[1][1])
                || (s[1][0] >= s[0][0] && s[1][1] <= s[0][1])
            {
                return true;
            } else {
                return false;
            }
        })
        .collect();

    println!(
        "Assignments fully containing another assignment: {}",
        exc1.len()
    );

    let exc2: Vec<_> = sections
        .iter()
        .filter(|s| {
            return (s[0][0]..(s[0][1] + 1)).contains(&s[1][0])
                || (s[0][0]..(s[0][1] + 1)).contains(&s[1][1])
                || (s[1][0]..(s[1][1] + 1)).contains(&s[0][0])
                || (s[1][0]..(s[1][1] + 1)).contains(&s[0][1]);
        })
        .collect();

    println!(
        "Assignments with any overlapping assignment: {}",
        exc2.len()
    );

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn get_section_list(s: &str) -> Vec<i32> {
    return s.split('-').map(|n| n.parse::<i32>().unwrap()).collect();
}
