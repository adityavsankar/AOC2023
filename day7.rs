use std::{collections::HashMap, cmp::Ordering};

pub fn solve(input:&Vec<(String,usize)>, joker:bool) -> usize {
    let mut sorted: Vec<(String,usize,usize)> = Vec::new();

    for (hand,bid) in input {
        let mut map: HashMap<char,usize> = HashMap::new();

        for card in hand.chars() {
            *(map.entry(card).or_insert(0)) += 1;
        }

        let mut hand_type: usize;
        let mut freq: Vec<_> = map.values().collect();
        freq.sort_unstable_by(|a,b| b.cmp(a));
        let &temp = freq[0];
        let n = freq.len();
        let j = *(map.get(&'J').unwrap_or(&0));
        
        match n {
            1 => hand_type = 7,
            2 => hand_type = if temp == 4 {6} else {5},
            3 => hand_type = if temp == 3 {4} else {3},
            4 => hand_type = 2,
            _ => hand_type = 1,
        }
        
        if joker {
            match hand_type {
                6 => hand_type = if j == 1 || j == 4 {7} else {6},
                5 => hand_type = if j == 2 || j == 3 {7} else {5},
                4 => hand_type = if j == 1 || j == 3 {6} else {4},
                3 => hand_type = if j == 2 {6} else if j == 1 {5} else {3},
                2 => hand_type = if j == 2 || j == 1 {4} else {2},
                1 => hand_type = if j == 1 {2} else {1},
                _ => (),
            }
        }
        sorted.push((hand.to_owned(),*bid,hand_type));
    }

    let card_value = |card:char|
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => if joker {1} else {11},
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            '1' => 1,
            _ => 0,
        };
    
    sorted.sort_unstable_by(|(a,_,c),(d,_,f)| 
    f.cmp(c)
    .then_with(|| {
        let x = d.as_bytes();
        let y = a.as_bytes();
        let mut z = Ordering::Equal;
        for i in 0..5 {
            let diff = card_value(x[i] as char) - card_value(y[i] as char);
            if diff > 0 {
                z = Ordering::Greater;
                break;
            }
            else if diff < 0 {
                z = Ordering::Less;
                break;
            }
        }
        z
    }));

    let mut result = 0;

    for (i,(_,b,_)) in sorted.iter().enumerate() {
        result += (sorted.len()-i)* *b;
    }
    result
}

fn main() {    
    let input: Vec<(String,usize)> = include_str!("input.txt")
    .lines()
    .map(|line| {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand = hand.to_string();
        let bid = bid.parse().unwrap();
        (hand,bid)
    })
    .collect();
  
    println!("Part 1: {}\nPart 2: {}",solve(&input,false),solve(&input,true));
}
