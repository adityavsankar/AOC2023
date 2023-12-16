use std::{collections::HashMap, usize};

fn calc(s: &str, n: &Vec<usize>, i: usize, b: usize, dp:&mut HashMap<(String,usize,usize),usize>) -> usize {
    if dp.contains_key(&(s.to_string(),i,b))  {
        return *dp.get(&(s.to_string(),i,b)).unwrap();
    }
    let mut x = 0;
    let mut y = 0;
    if s.is_empty() {
        return if (b == 0 && i >= n.len()) || (i == n.len()-1 && b == n[i]) {1} else {0}; 
    }
    else if s.starts_with(".") {
        if i >= n.len() || b == n[i] {
            return calc(&s[1..],n,i+1,0,dp);
        }
        if b == 0 {
            return calc(&s[1..],n,i,b,dp);
        }
        return 0;
    }
    else if s.starts_with("#") {
        if i < n.len() && b < n[i] {
            return calc(&s[1..],n,i,b+1,dp);
        }
        return 0;
    }
    if i >= n.len() || b == n[i] {
        x = calc(&s[1..],n,i+1,0,dp);
    }
    else if b == 0 {
        x = calc(&s[1..],n,i,0,dp); 
    }
    if i < n.len() && b < n[i] {
        y = calc(&s[1..],n,i,b+1,dp);
    }
    dp.insert((s.to_string(),i,b), x+y);
    x+y
}

pub fn solve(n: usize) -> usize {
    let input: Vec<(&str,Vec<usize>)> = include_str!("input.txt")
    .lines()
    .map(|line| {
        let x = line.split_once(" ").unwrap();
        (x.0,x.1.split(",").map(|n| n.parse().unwrap()).collect())
    })
    .collect();

    let ans: usize = input
    .iter()
    .fold(0, |acc, x| {
        let news = &[x.0,"?"].repeat(n).concat();
        let temp = &news[0..news.len()-1];
        let newn = &x.1.repeat(n);
        let mut dp: HashMap<(String,usize,usize),usize> = HashMap::new();
        acc+calc(temp,newn,0,0,&mut dp)
    });
    ans
}

fn main() {
    let (part1,part2) = (solve(1),solve(5));
    println!("Part 1: {}\nPart 2: {}",part1,part2);
}
