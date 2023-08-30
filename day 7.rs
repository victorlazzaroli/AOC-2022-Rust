use std::fs::read_to_string;
use itertools::Itertools;


use regex::Regex;
use socarel::NodeContent;
#[derive(Debug, PartialEq)]
enum CommandE {
    CD,
    LS,
    PARENT
}


#[derive(Debug, Clone, PartialEq)]
enum FileType {
    DIR,
    FILE
}

struct Command {
    command: CommandE,
    params: Vec<String>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    file_type: Option<FileType>,
    dimension: Option<usize>,
}

impl NodeContent for File {
    fn new (content: &str) -> Option<Self> {
            Some(Self {
                name: content.to_string(),
                file_type: None,
                dimension: None,
            })
    }

    fn get_val(&self) -> &str {
        &self.name
    }
 
    fn gen_content(&self) -> String {
        format!("{:?} - {}: {:?}", self.file_type, self.name, self.dimension)
    }
}

fn parser(line: &str) {
    let re = Regex::new(r#"\$ (\w+) ?(\.{2}|/|\w*|)?"#).unwrap();
    let caps = re.captures(line);
    match caps {
        Some(caps) => 
        { 
            if let Some(cmd) = caps.get(1) {
                
            }

            if let Some(params) = caps.get(2) {

            }
        },
        None => None
    }


}

fn main() {
    let inputs = read_to_string("inputs.txt").expect("file with inputs");
    // inputs.lines().batching(|it| )
    inputs.lines().for_each(|line| parser(line));
}
