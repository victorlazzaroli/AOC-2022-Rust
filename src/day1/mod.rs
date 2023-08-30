use std::fs;
use std::path::Path;

pub fn solution1() {
    println!(" - Soluzione giorno 1 - \n\n");
    let path = Path::new("./src/day1/calories.txt");
    let calories = fs::read_to_string(path).expect("Mi aspettavo un file di merda");
    let mut elves = Vec::new();
    let mut max_elve: (usize, i32) = (0, 0);

    elves.push(0);

    for line in calories.lines() {
        if line.is_empty() {
            let elves_len = elves.len();
            if let Some(last_elve) = elves.last_mut() {
                if *last_elve > max_elve.1 {
                    max_elve = (elves_len, *last_elve);
                }
            }
            elves.push(0);
        } else {
            let my_int = line.parse::<i32>().unwrap();
            if let Some(last_elve) = elves.last_mut() {
                *last_elve = *last_elve + my_int;
            }
        }

    }

    elves.sort_by(|a, b| b.cmp(a));
    let greatest_three_elves = &elves[..3];
    let greatest_three: i32 = greatest_three_elves.iter().sum();
    println!("Max elve is : {:?}", max_elve);
    println!("Greatest 3 : {:?}", greatest_three_elves);
    println!("Sum of Greatest 3 : {}", greatest_three);

    println!("\n\n - Fine giorno 1 - \n\n");
}