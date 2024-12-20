use std::io;

fn string2vec(l: &String) -> Vec<i32> { 
    l.split_whitespace().map(|a| a.parse().unwrap()).collect()
}

fn diff(v: &Vec<i32>) -> Vec<i32> {
    v.windows(2).map(|w| w[1] - w[0]).collect()
}

fn is_monotonic(v: &Vec<i32>) -> bool {
    let pos = v.iter().all(|x| x > &0);
    let neg = v.iter().all(|x| x < &0);
    pos || neg
}

fn is_small_steps(v: &Vec<i32>) -> bool {
    v.iter().all(|&x| (x.abs() >= 1) && (x.abs() <= 3))
}

fn solve1(v: &Vec<i32>) -> bool {
    let deltas = diff(v);
    is_monotonic(&deltas) && is_small_steps(&deltas)
}

fn solve2(v: &Vec<i32>) -> bool {
    if solve1(&v) {return true};
    for i in 0..v.len() {
        let mut vi = v.clone();
        vi.remove(i);
        if solve1(&vi) {return true};
    }
    false
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().flatten().collect();
    let mut tot1 = 0;
    let mut tot2 = 0;
    for line in lines {
        let v = string2vec(&line);
        tot1 += solve1(&v) as i32;
        tot2 += solve2(&v) as i32;
    }
    println!("Part 1: {}", tot1);
    println!("Part 2: {}", tot2);
}
