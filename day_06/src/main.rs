use std::collections::HashSet;
use std::fs;
use std::time::Instant;
fn main() {
    // get string from input
    let start = Instant::now();

    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    //create vector of chars
    let slice = contents.chars().collect::<Vec<char>>();

    //ok, normally I like to get both first and second exercise results, but this is a fun Match for practice
    let start_marker: usize = match slice.len() {
        0..=100 => 4,
        _ => 14,
    };

    //loop over slice with windows
    for (i, v) in slice.windows(start_marker).enumerate() {
        let hs: HashSet<_> = v.iter().collect(); //create hashset from vector
        if hs.len() == start_marker {
            // if all are different note the index
            println!(
                "\nFirst occurence of {} different characters in a row occurs after {} characters",
                start_marker,
                i + start_marker
            );
            break;
        }
    }

    let duration = start.elapsed();
    println!("\n\nTime elapsed is: {:?}", duration);
}
