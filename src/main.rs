use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap, HashSet};


fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut emptyLineNumber = 0;

    for (i, line) in reader.lines().enumerate() {
        if line.unwrap().is_empty() {
            emptyLineNumber = i;
        }
    }

    let reader = BufReader::new(File::open("input.txt").unwrap());

    let lines: Vec<String>= reader.lines().map(|l| l.unwrap()).collect();
    let stackRegion = &lines[0..emptyLineNumber];
    let stackNumber = stackRegion[emptyLineNumber - 1].split_whitespace().collect::<Vec<&str>>().len();

    let mut stacks :Vec<Vec<char>> = Vec::new();

    for i in 0..stackNumber {
        stacks.push(Vec::new());
    }

    for  line in stackRegion[0..(stackRegion.len()-1)].iter().rev() {
        let mut characterIndex = 1;
        let mut stackIndex = 0;

        while characterIndex < line.len() {

            if line.chars().nth(characterIndex).unwrap() != ' '{
                stacks[stackIndex].push(line.chars().nth(characterIndex).unwrap());
            }

            characterIndex += 4;
            stackIndex += 1;
        }
    }

    for line in &lines[emptyLineNumber+1..] {

        println!("line: {}", line);

        let elements = line.split_whitespace().collect::<Vec<&str>>();
        let moveAmount = elements[1].parse::<i32>().unwrap();
        let moveFrom = elements[3].parse::<i32>().unwrap() - 1;
        let moveTo = elements[5].parse::<i32>().unwrap() - 1;

        for _ in 0..moveAmount {
            let element = stacks[moveFrom as usize].pop().unwrap();
            stacks[moveTo as usize].push(element);
        }

        for s in &stacks{

            let mut printstack: String = "[".to_string();
            for e in s{
                printstack.push(*e);
            }
            printstack +="]";

            println!("stack: {}", printstack);

        }
    }

    //let numberOfStacks = lines[emptyLineNumber -1].unwrap().split(" ").collect().len();
}