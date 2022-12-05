use std::fs;

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let v: Vec<String> = contents
        .split("\n")
        .filter_map(|w| w.parse().ok())
        .collect();

    let mut n: u32 = 0;
    let mut calorie_list: Vec<u32> = vec![];

    for (i, el) in v.iter().enumerate() {
        if el == "" {
            calorie_list.push(n);
            n = 0;
        } else {
            let c: u32 = el.parse().unwrap();
            n = n + c;
            if i == v.len() - 1 {
                calorie_list.push(n);
            }
        };
    }

    println!(
        "strongest elf carries {} calories",
        calorie_list.iter().max().unwrap()
    );

    calorie_list.sort();
    calorie_list.reverse();

    print!(
        "3 strongest elves carry a combined {} calories\n",
        calorie_list[0] + calorie_list[1] + calorie_list[2]
    );
}
