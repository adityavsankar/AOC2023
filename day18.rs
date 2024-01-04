fn parse(input: &str, part2: bool) -> Vec<(char, isize)> {
    let dig_plan: Vec<(char,isize)> = match part2 {
        true => {
            input
            .lines()
            .map(|x|{ 
                let color = x.split(" ").collect::<Vec<_>>()[2];
                let dist = isize::from_str_radix(&color[2..7],16).unwrap();
                let dir = match color.chars().nth(7).unwrap() {
                    '0' => 'R',
                    '1' => 'D',
                    '2' => 'L',
                    '3' => 'U',
                     _  => 'X',
                };
                (dir,dist)
            })
            .collect()
        },
        false => {
            input
            .lines()
            .map(|x|{ 
                let temp: Vec<_> = x.split(" ").collect();
                let dir: char = temp[0].chars().next().unwrap();
                let dist: isize = temp[1].parse().unwrap();
                (dir,dist)
            })
            .collect()
        },
    };
    dig_plan
}

fn area(dig_plan: &Vec<(char,isize)>) -> isize {
    let (mut prev_x, mut prev_y, mut result): (isize,isize,isize) = (0,0,0);
    for (dir,len) in dig_plan {
        let (x,y);
        match dir {
            'U' => (x,y) = (prev_x + len, prev_y),
            'D' => (x,y) = (prev_x - len, prev_y),
            'L' => (x,y) = (prev_x, prev_y - len),
            'R' => (x,y) = (prev_x, prev_y + len),
             _  => (x,y) = (0,0),
        }
        result += (prev_y + y) * (prev_x - x) + len;
        (prev_x,prev_y) = (x,y);
    }
    r/2 + 1
}

fn solve(input: &str) -> (isize,isize) {
    let part1 = area(&parse(&input, false));
    let part2 = area(&parse(&input, true));
    (part1,part2)
}

fn main() {
    let input = include_str!("input.txt");
    let (part1, part2) = solve(input);
    println!("Part 1: {}\nPart 2: {}",part1,part2);
}
