use ascii_converter::{decimals_to_string, string_to_decimals};
use std::{fs::read_to_string, mem::replace};

#[derive(Clone, Debug, PartialEq)]
struct RuckSack<'a> {
    num_of_el: usize,
    priority_left: Vec<i32>,
    priority_right: Vec<i32>,
    priority_rucksack: Vec<i32>,
    left: &'a str,
    right: &'a str,
}

fn input_into_compartmens(input: &str) -> RuckSack {
    let compartments = input.split_at(input.chars().count() / 2);
    let mut priority_left = vec![0; 53];
    let mut priority_right = vec![0; 53];
    let mut priority_rucksack = vec![0; 53];

    let compartment_priorities_left = string_to_priority(compartments.0);
    let compartment_priorities_right = string_to_priority(compartments.1);

    for comp_pr_left in compartment_priorities_left {
        let curr_val = priority_left[comp_pr_left as usize];
        let _ = replace(&mut priority_left[comp_pr_left as usize], curr_val + 1);
        let _ = replace(&mut priority_rucksack[comp_pr_left as usize], curr_val + 1);
    }
    for comp_pr_right in compartment_priorities_right {
        let curr_val = priority_right[comp_pr_right as usize];
        let _ = replace(&mut priority_right[comp_pr_right as usize], curr_val + 1);
        let _ = replace(&mut priority_rucksack[comp_pr_right as usize], curr_val + 1);
    }

    RuckSack {
        num_of_el: compartments.0.chars().count() + compartments.1.chars().count(),
        priority_left,
        priority_right,
        priority_rucksack,
        left: compartments.0,
        right: compartments.1,
    }
}

fn string_to_priority(s: &str) -> Vec<u8> {
    string_to_decimals(s)
        .unwrap()
        .iter()
        .map(|byte| byte_to_priority(*byte))
        .collect()
}

fn byte_to_priority(byte: u8) -> u8 {
    if byte > 96 {
        byte - 96
    } else {
        byte - 64 + 26
    }
}

fn priority_to_byte(priority: u8) -> u8 {
    if priority > 26 {
        priority + 64 - 26
    } else {
        priority + 96
    }
}

fn priority_to_string(bytes: &Vec<u8>) -> String {
    decimals_to_string(bytes).unwrap()
}

// Calcola la priorità totale per ogni singola lettera che si trova sia a sx che a dx del ruckstack
fn common_items_priorities(priority_left: &Vec<i32>, priority_right: &Vec<i32>) -> Vec<i32> {
    priority_left
        .iter()
        .zip(priority_right.iter())
        .enumerate()
        .filter(|(index, left_right)| left_right.0 * left_right.1 != 0)
        .map(|(index, left_right)| (index as i32))
        .collect::<Vec<i32>>()
}

fn part1(rucksacks: &Vec<RuckSack>) {
    let test = rucksacks
        .iter()
        .map(|ruckstack| {
            common_items_priorities(&ruckstack.priority_left, &ruckstack.priority_right)
        })
        .collect::<Vec<Vec<i32>>>();

    let mut somma_totale = 0;
    for elements in test {
        for el in elements {
            somma_totale += el;
            println!("el: {}, totale attuale: {}", el, somma_totale);
        }
    }
}

fn part2(rucksacks: &Vec<RuckSack>) {
    let mut gruppi = Vec::new();
    let gruppiType ;
    let mut sommaTotale: u32 = 0;
    for i in (0..rucksacks.len()).step_by(3) {
        let primo = &rucksacks.get(i).unwrap().priority_rucksack;
        let secondo = &rucksacks.get(i+1).unwrap().priority_rucksack;
        let terzo = &rucksacks.get(i+2).unwrap().priority_rucksack;
        for j in (0..53) {
           if primo[j] * secondo[j] * terzo[j] != 0 {
               gruppi.push(j as u8);
           } else {
            continue;
           }
        }
    }
    gruppiType = decimals_to_string(&(gruppi.iter().map(|el| priority_to_byte(*el as u8)).collect())).unwrap();
    for gruppo in &gruppi {
        sommaTotale += *gruppo as u32;
    }
    println!("{:?}", gruppi);
    println!("Badges {:?}", gruppiType);
    println!("Somma {:?}", sommaTotale);
}

fn main() {
    let inputs = read_to_string("inputs.txt").expect("file with inputs");
    let rucksacks: Vec<RuckSack> = inputs
        .lines()
        .map(|input| input_into_compartmens(input))
        .collect();

    // part1(&rucksacks);
    part2(&rucksacks);
}
