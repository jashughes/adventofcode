use std::io;
use regex::Regex;
use itertools::Itertools;

struct HistList {
    left: Vec<u32>,
    right: Vec<u32>
}

impl HistList {
    fn distance(&self) -> u32 {
        self.left.iter()
                 .zip(self.right.iter())
                 .map(|(a, b)| a.abs_diff(*b))
                 .sum()
    }
    fn similarity(&self) -> u32 {
        let counts = self.right.iter().counts();
        let mut score = 0;
        for l in &self.left {
            score += l * (*counts.get(&l).unwrap_or(&0) as u32);
        }
        score
    }
}

impl TryFrom<Vec<String>> for HistList {
    type Error = ();

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        let re = Regex::new("([0-9]+) +([0-9]+)").unwrap();
        let mut x1 = vec![];
        let mut x2 = vec![];
        for line in lines {
            let xi = re.captures(&line).unwrap();
            let xi1 = xi[1].parse().unwrap();
            let xi2 = xi[2].parse().unwrap();
            x1.push(xi1);
            x2.push(xi2);
        }
        x1.sort();
        x2.sort();
        Ok(HistList { left: x1, right: x2 })
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().flatten().collect();
    let points: HistList = lines.try_into().unwrap();
    
    let pt1 = points.distance();
    let pt2 = points.similarity();
    
    println!("Part 1: {pt1}");
    println!("Part 2: {pt2}");
}
