use std::collections::HashMap;

fn gcd(mut a: usize,mut b: usize) -> usize {
    let mut temp;
    while b!=0 {
        temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(dists: Vec<usize>) -> usize {
    let mut lcm = dists[0];
    for i in 1..dists.len() {
        lcm = (dists[i]*lcm)/gcd(dists[i],lcm);
    }
    lcm
}

pub fn solve(input: &Vec<&str>,part2: bool) -> usize {
    let path: Vec<char> = input[0].chars().collect();

    let mut map: HashMap<&str,(&str,&str)> = HashMap::new();
    let mut routes: Vec<&str> = vec![];

    let start = if part2 {"A"} else {"AA"};
    let end = if part2 {"Z"} else {"ZZ"};

    for i in 2..input.len() {
        let temp = input[i]
        .split_once(" = ")
        .unwrap();

        let key = temp.0;

        let value = temp.1
        .trim_start_matches("(")
        .trim_end_matches(")")
        .split_once(", ")
        .unwrap();

        if key.ends_with(start) {
            routes.push(key);
        }

        map.insert(key,(value.0,value.1));
    }

    let mut dist;
    let n = path.len();
    let mut i;
    let mut dists:Vec<usize> = vec![];

    for j in 0..routes.len() {
        i = 0;
        dist = 0;
        while !routes[j].ends_with(end) {
            let x = path[i];
            let fork = map.get(&routes[j]).unwrap();
            match x {
                'L' => routes[j] = fork.0,
                'R' => routes[j] = fork.1,
                _ => (),
            };
            i = (i+1)%n;
            dist += 1;
        }
        dists.push(dist);
    }
    
    lcm(dists)
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt")
    .lines()
    .collect();

    let part1 = solve(&input,false);
    let part2 = solve(&input,true);
    println!("Part 1: {}\nPart 2: {}",part1,part2);
}
