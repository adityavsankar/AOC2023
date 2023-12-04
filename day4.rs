fn main() {
    
    let start = std::time::Instant::now();
    
    let input: Vec<String>  = 
    include_str!("input.txt")
    .lines()
    .map(|line| line.to_string())
    .collect();
    
    let mut part1:i32 = 0;
    let mut arr: [i32;197] = [1;197];
    arr[0] = 0;
    let mut i:usize = 1;
    
    for x in input {
        let mut matches:i32 = 0;
        
        let card: Vec<String> = x
        .split(": ")
        .map(|a| a.to_string())
        .collect();
        
        let parts: Vec<String> = card[1]
        .split("|")
        .map(|b| b.to_string())
        .collect();
        
        let winning: Vec<i32> = parts[0]
        .split(" ")
        .map(|val| val.parse().unwrap_or(-1))
        .collect();
        
        let given: Vec<i32> = parts[1]
        .split(" ")
        .map(|val| val.parse().unwrap_or(-1))
        .collect();
        
        for p in given {
            for q in &winning {
                if p==*q && p>0 && *q>0 {
                    matches+=1;
                }
            }
        }
        
        part1 += (1<<matches)>>1;
        
        for r in i+1..=i+(matches as usize) {
            arr[r] += arr[i];
        }
        
        i+=1;
    }
    let part2: i32 = arr.iter().sum();
    println!("Points: {}\nScratch Cards: {}\nTime taken: {} us",
    part1,part2,start.elapsed().as_micros());
}
