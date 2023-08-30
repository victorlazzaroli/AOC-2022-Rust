use std::fs;
use std::path::Path;

trait ShapeCmp {
    fn shape_cmp(&self, opponent: &ShapePoints) -> RoundPoints;
}

#[derive(Clone, Debug, PartialEq)]
enum RoundPoints {
    Win = 6,
    Draw = 3,
    Lost = 0
}

impl ShapeCmp for ShapePoints {
    fn shape_cmp(&self, opponent: &ShapePoints) -> RoundPoints {
    
        if *self == *opponent {
            return RoundPoints::Draw;
        }
    
        if (*self == ShapePoints::Rock && *opponent == ShapePoints::Scissors) ||
        (*self == ShapePoints::Scissors && *opponent == ShapePoints::Paper) || 
        (*self == ShapePoints::Paper && *opponent == ShapePoints::Rock) 
        {
            return RoundPoints::Win;
        }
    
        return RoundPoints::Lost;
    
    } 
}


#[derive(Debug, Clone, PartialEq)]
enum ShapePoints {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

fn calc_points(s: &ShapePoints, r: &RoundPoints) -> u32 {
    (s.clone()) as u32 + (r.clone()) as u32
}

fn shape_forRes(opponent: &ShapePoints, expectedResult: &RoundPoints) -> Option<ShapePoints> {
    if (*expectedResult == RoundPoints::Draw) {
        return Some(opponent.clone());
    }

    if (*expectedResult == RoundPoints::Win) {
        match *opponent {
            ShapePoints::Rock => return Some(ShapePoints::Paper),
            ShapePoints::Paper => return Some(ShapePoints::Scissors),
            ShapePoints::Scissors => return Some(ShapePoints::Rock),
        }
    }

    if (*expectedResult == RoundPoints::Lost) {
        match *opponent {
            ShapePoints::Rock => return Some(ShapePoints::Scissors),
            ShapePoints::Paper => return Some(ShapePoints::Rock),
            ShapePoints::Scissors => return Some(ShapePoints::Paper),
        }
    } else {
        None
    }
}

fn conversion(letter: &str) -> Option<ShapePoints> {
    let conversion_map = [
        ("A", ShapePoints::Rock), ("B", ShapePoints::Paper), ("C", ShapePoints::Scissors),
        ("Y", ShapePoints::Paper), ("X", ShapePoints::Rock), ("Z", ShapePoints::Scissors)
     ];

     if let Some(val) = conversion_map.iter().find(|el| el.0 == letter) {
        return Some(val.1.clone());
     } else {
        return None;
     }
}
fn conversion_part2(letter: &str) -> Option<RoundPoints> {
    let conversion_map = [
        ("Y", RoundPoints::Draw), ("X", RoundPoints::Lost), ("Z", RoundPoints::Win)
     ];

     if let Some(val) = conversion_map.iter().find(|el| el.0 == letter) {
        return Some(val.1.clone());
     } else {
        return None;
     }
}

fn part1(me: &Vec<Option<ShapePoints>>, opponent: &Vec<Option<ShapePoints>>) {
    let it = me.iter().zip(opponent.iter());
    let mut roundResults = Vec::new();
    let mut roundMyPoints = Vec::new();
    let mut somma = 0;

    for (i, (x, y)) in it.enumerate() {
        let myShape = x.clone().unwrap();
        let opponentShape = y.clone().unwrap();
        roundResults.push(myShape.clone().shape_cmp(&opponentShape));
        let currPoints = calc_points(&myShape, roundResults.last().unwrap());
        roundMyPoints.push(calc_points(&myShape, roundResults.last().unwrap()));
        somma += currPoints;
    }

    println!("Reults: {:?}", roundResults);
    println!("Points: {:?}", roundMyPoints);
    println!("Somma: {somma}");
}

fn part2() {
    let input_file = fs::read_to_string("./src/day2/rounds.txt").expect("Mi aspettavo un file di merda");
    let rounds: Vec<(Option<ShapePoints>, Option<ShapePoints>, Option<RoundPoints>)> = input_file
                                .lines()
                                .map(
                                    |el| el.split_whitespace()
                                                .collect::<Vec<&str>>()
                                ).map(|el| (
                                    conversion(el[0]), shape_forRes(&conversion(el[0]).unwrap(), &conversion_part2(el[1]).unwrap()), conversion_part2(el[1])
                                ))
                                .collect::<Vec<(Option<ShapePoints>, Option<ShapePoints>, Option<RoundPoints>)>>();
    let roundResults: Vec<u32> = rounds
        .iter()
        .map(|round| calc_points(&round.1.clone().unwrap(), &round.2.clone().unwrap()))
        .collect();
    
    let totalResult: u32 = roundResults.iter().sum();

    println!("Rounds: {:?}", rounds);
    println!("Round results: {:?} \n Total: {}", roundResults, totalResult);
}

pub fn solution2() {
    println!(" - Soluzione giorno 2 - \n\n");
    let path = Path::new("./src/day2/rounds.txt");
    let rounds = fs::read_to_string(path).expect("Mi aspettavo un file di merda");
    let mut opponent = Vec::new();
    let mut me = Vec::new();

    for line in rounds.lines() {
       let round: Vec<&str> = line.split(" ").collect();
       let round_opponent = round.get(0);
       let round_me = round.get(1);

       if round_opponent.is_some() && round_me.is_some() {
        opponent.push(conversion(round_opponent.unwrap().clone()));
        me.push(conversion(round_me.unwrap().clone()));
       }
    }
    //println!("Me: {:?}", me);
    //println!("Opponent {:?}", opponent);

    //part1(&me, &opponent);
    part2();

    

    println!("\n\n - Fine giorno 2 - \n\n");
}