use std::{fs::read_to_string, collections::HashSet};

#[derive(Clone, Debug, PartialEq)]
struct sections_assignment {
    elve_1: Vec<u8>,
    elve_2: Vec<u8>,
}

fn string_to_assignment(stringa: &str) -> Vec<u8> {
    let lower_bound: u8;
    let upper_bound: u8;

    let v = stringa
        .split('-')
        .map(|el| el.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    lower_bound = v[0];
    upper_bound = v[1];

    (lower_bound..(upper_bound + 1)).collect()
}

fn part1(sections: &Vec<sections_assignment>) {
    let mut unfair_pairs = 0;
    for section in sections {
        let mut insieme: HashSet<u8> = HashSet::from_iter(section.elve_1.clone());
        let before = insieme.len();
        insieme.extend(section.elve_2.clone().iter());
        let after = insieme.len();
        if (before == after) || (after == section.elve_2.len()) {
            println!("DENTROOO\n elve1: {:?}\n elve2: {:?}", &section.elve_1, &section.elve_2);
            unfair_pairs += 1;
        }
        
    }

    println!("{unfair_pairs}");
}

fn part2(sections: &Vec<sections_assignment>) {
    let mut overlap_pairs = 0;
    for section in sections {
        let mut insieme: HashSet<u8> = HashSet::from_iter(section.elve_1.clone());
        let before = insieme.len();
        insieme.extend(section.elve_2.clone().iter());
        let after = insieme.len();
        if after < (before + section.elve_2.len()) {
            println!("DENTROOO\n elve1: {:?}\n elve2: {:?}", &section.elve_1, &section.elve_2);
            overlap_pairs += 1;
        }
        
    }

    println!("{overlap_pairs}");
}

fn main() {
    let inputs = read_to_string("inputs.txt").expect("file with inputs");
    let test = inputs
        .lines()
        .map(|pairString| {
            let moment = pairString
                .split(",")
                .map(|section| string_to_assignment(section))
                .collect::<Vec<Vec<u8>>>();

            sections_assignment {
                elve_1: moment[0].clone(),
                elve_2: moment[1].clone(),
            }
        })
        .collect::<Vec<sections_assignment>>();

    println!("{:?}", test);
    // part1(&test);
    part2(&test);
}
