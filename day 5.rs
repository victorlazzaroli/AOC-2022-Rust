use std::fs::read_to_string;

use regex::Regex;

fn separate_cargo_from_actions(input: &str) -> (&str, &str) {
    let mut cargo_composition;
    let mut actions;
    let actionsIdx;

    actionsIdx = input.find("\r\n\r\n").unwrap();
    cargo_composition = &input[0..actionsIdx];
    actions = &input[actionsIdx..];

    return (cargo_composition, actions);
}

fn get_num_of_stacks(input: &str) -> usize {
    input.split_whitespace().map(|e| e.trim()).last().unwrap().parse::<usize>().unwrap()
}

fn build_cargo(num_of_stacks: usize, line: &str) -> Vec<String> {
    let mut stacks: Vec<String> = Vec::new();
    stacks.push(line.chars().nth(1).unwrap().to_string().clone());
    for i in (1..num_of_stacks) {
        let car = line.chars().nth(1+i*4).unwrap().to_string();
        stacks.push(car.clone());
    }
    println!("{:?}", stacks);
    stacks
}

fn traspose_cargo (num_col: usize, cargo: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut new_cargo: Vec<Vec<String>>= Vec::new(); 
    for col in (0..num_col) {
        let mut rowVec: Vec<String> = Vec::new(); 
        for row in (0..cargo.len()).rev(){
            let value = cargo.get(row).unwrap().get(col).unwrap();
            if (value.trim() != "") {
                // println!("Cargo row: {:?}", value);
                rowVec.push(value.clone());
            }
        }
        new_cargo.push(rowVec.clone());
    }

    new_cargo
    // cargo
}

fn build_move(line: &str) -> (u16, u16, u16) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let caps = re.captures(line);
    if let Some(captures) = caps {
        (captures.get(1).unwrap().as_str().parse::<u16>().unwrap(), captures.get(2).unwrap().as_str().parse::<u16>().unwrap(), captures.get(3).unwrap().as_str().parse::<u16>().unwrap())
        // captures.iter().for_each(|capture| println!("{:?}", capture.unwrap()));
    } else {
        (0,0,0)
    }
}

fn action_move(num_of_packs: u16, stack_from: u16, stack_to: u16, cargo: &mut Vec<Vec<String>>) {
    if num_of_packs == 0 || stack_from == 0 || stack_to == 0 {
        return;
    } else {
        for pack in (0..num_of_packs) {
            let el = cargo[(stack_from as usize) - 1 ].pop().unwrap();
            cargo[(stack_to as usize) - 1 ].push(el);
        }
    }
}

fn action_move9001(num_of_packs: u16, stack_from: u16, stack_to: u16, cargo: &mut Vec<Vec<String>>) {
    if num_of_packs == 0 || stack_from == 0 || stack_to == 0 {
        return;
    } else {
        let mut block = Vec::new();
        for pack in (0..num_of_packs) {
            let el = cargo[(stack_from as usize) - 1 ].pop().unwrap();
            block.push(el);
        }
        block.reverse();
        cargo[(stack_to as usize) - 1 ].append(&mut block);
    }
}

fn get_stack_lasts(cargo: Vec<Vec<String>>) -> String {
    let mut lasts: String= String::new();

    for stack in cargo {
        let last_pack = stack.last();
        if let Some(pack) = last_pack {
            lasts.push_str(pack);
        }
    }

    lasts
}


fn main() {
    let inputs = read_to_string("inputs.txt").expect("file with inputs");
    let (cargo_composition_input, cargo_movements_input) = separate_cargo_from_actions(&inputs);
    let num_of_stacks = get_num_of_stacks(cargo_composition_input.lines().last().unwrap());
    let mut cargo = cargo_composition_input.lines().map(|line| build_cargo(num_of_stacks, line)).collect::<Vec<Vec<String>>>();
    cargo.pop();
    println!("Cargo POP: {:?}\n", cargo);
    cargo = traspose_cargo(num_of_stacks, cargo);
    println!("Cargo TRASPOSE: {:?}\n", cargo);
    let moves: Vec<(u16, u16, u16)> = cargo_movements_input.lines().map(|line| build_move(line)).collect();
    
    for movement in &moves {

        println!("Move: {:?}", movement);
        // action_move(movement.0, movement.1, movement.2, &mut cargo);
        action_move9001(movement.0, movement.1, movement.2, &mut cargo);
        println!("Cargo: {:?}\n", cargo);
    }
    
    let stack_lasts = get_stack_lasts(cargo);
    println!("Cargo lasts: {:?}\n", stack_lasts);

}


