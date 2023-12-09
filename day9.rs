fn gen_diff(v: &Vec<i32>) -> Vec<i32> {
    let mut diff: Vec<i32> = vec![];
    for i in 0..v.len()-1 {
        diff.push(v[i+1]-v[i]);
    }
    diff
}

pub fn solve(input: Vec<String>) -> (i32,i32) {
    let mut forward: i32 = 0;
    let mut backward: i32 = 0;
    let mut f;
    let mut b;
    let mut diffs: Vec<Vec<i32>>;
    
    for x in input {
        f = 0;
        b = 0;
        diffs = vec![];

        let numbers: Vec<i32> = x.split(" ")
        .map(|num| num.parse().unwrap())
        .collect();
    
        diffs.push(numbers);

        while !diffs[diffs.len()-1].iter().all(|n| *n == 0) {
            diffs.push(gen_diff(&diffs[diffs.len()-1]));
        }

        for (i,y) in diffs.iter().enumerate() {
            f += y[y.len()-1];
            b = if i&1 == 1 {b-y[0]} else {b+y[0]};
        }

        forward += f;
        backward += b;
    }
    (forward,backward)
}

fn main() {   
    let input: Vec<String> = include_str!("input.txt")
    .lines()
    .map(|line| line.to_string())
    .collect();

    let result = solve(input);
    println!("Forward Extrapolation: {}\nBackward Extrapolation: {}",
    result.0,result.1);
}
