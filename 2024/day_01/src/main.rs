use std::io;
use regex::Regex;
use itertools::Itertools;

fn main() {
    let lines = io::stdin().lines();
    let re = Regex::new("([0-9]+) +([0-9]+)").unwrap();
    let mut x1 = vec![];
    let mut x2 = vec![];
    for line in lines {
        let line1 = line.unwrap();
        let xi = re.captures(&line1).unwrap();
        let xi1 = xi[1].parse::<i32>().unwrap();
        let xi2 = xi[2].parse::<i32>().unwrap();
        x1.push(xi1);
        x2.push(xi2);
    }

    x1.sort();
    x2.sort();
    let x3: u32 = x1.iter().zip(x2.iter()).map(|(a, b)| a.abs_diff(*b)).sum();
    println!("Part 1: {x3}");

    let counts = x2.into_iter().counts();
    let mut score = 0;
    for x in x1 {
        
        score += x * (*counts.get(&x).unwrap_or(&0) as i32);
    }
    println!("Part 2: {score}");
}
