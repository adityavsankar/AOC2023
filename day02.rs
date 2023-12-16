fn main() {
    let start = std::time::Instant::now();
    let input: Vec<String> = 
    include_str!("input.txt")
    .lines()
    .map(|line| line.trim_start_matches("Game ").to_string())
    .collect();
    let (mut r, mut g, mut b) : (i32,i32,i32);
    let mut id_sum = 0;
    let mut power_sum = 0;
    for x in input {
        (r,b,g) = (0,0,0);
        let temp: Vec<String> =
        x.split(": ").
        map(|s| s.to_string())
        .collect();
        let id: i32 = temp[0].parse().unwrap();
        let mut i = 0;
        let game = temp[1].clone().into_bytes();
        while i<game.len() {
            let mut num = 0;
            while i<game.len() && (game[i] as char).is_digit(10) {
                num = num*10 + (game[i] as i32 - 48);
                i+=1;
            }
            if i+1 < game.len() {
                match game[i+1] as char {
                    'r' => r = if num>r {num} else {r},
                    'g' => g = if num>g {num} else {g},
                    'b' => b = if num>b {num} else {b},
                    _ => (),
                }
            }
            i+=1;
        }
        if r < 13 && g < 14 && b < 15 {
            id_sum += id;
        }
        power_sum += r*g*b;
    }
    println!("Sum of Ids: {}\nSum of Powers: {}\nTime Taken: {} us"
    ,id_sum,power_sum,start.elapsed().as_micros());
}
