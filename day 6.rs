use std::{fs::read_to_string, collections::HashSet};
fn has_duplicates(test: &[u8]) -> bool {
    let mut insieme = HashSet::new();

    insieme = HashSet::from_iter(test.iter().clone());

    insieme.len() != test.len()
}

fn main() {
    let inputs = read_to_string("inputs.txt").expect("file with inputs");
    let finestraN = inputs.as_bytes().windows(4).collect::<Vec<&[u8]>>();
    let indice = finestraN.iter().position(|window| !has_duplicates(window)).unwrap() + 4;
    let finestraNpart2 = inputs.as_bytes().windows(14).collect::<Vec<&[u8]>>();
    let indicePart2 = finestraNpart2.iter().position(|window| !has_duplicates(window)).unwrap() + 14;

    println!("Part1: {indice:?}");
    println!("Part2: {indicePart2:?}");
}
