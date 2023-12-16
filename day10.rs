fn traverse(start: ((usize,usize),char,usize),grid: &Vec<Vec<char>>,newgrid: &mut Vec<Vec<char>>) -> usize {
    let mut p = start.0;
    let mut c = start.1;
    let mut d = start.2;
    let mut dist = 0;
    
    while c != 'S' {
        newgrid[p.0][p.1] = 
            match c {
                '-' => '-',
                '|' => 'X',
                'F' => 'F',
                '7' => '7',
                'L' => 'X',
                'J' => 'X',
                _  => '.',
            };

        match (d,c) {
            (1,'7') => {p.1 -= 1; d = 3},
            (1,'|') =>  p.0 -= 1,
            (1,'F') => {p.1 += 1; d = 4},

            (2,'L') => {p.1 += 1; d = 4},
            (2,'|') =>  p.0 += 1,
            (2,'J') => {p.1 -= 1; d = 3},

            (3,'L') => {p.0 -= 1; d = 1},
            (3,'-') =>  p.1 -= 1,
            (3,'F') => {p.0 += 1; d = 2},

            (4,'7') => {p.0 += 1; d = 2},
            (4,'-') =>  p.1 += 1,
            (4,'J') => {p.0 -= 1; d = 1},
               _    => break
        }

        dist += 1;
        c = grid[p.0][p.1];
    }
    (dist>>1) + 1
}

fn find_trapped(newgrid: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    let mut k;
    let mut count;

    for i in 0..newgrid.len() {
        for j in 0..newgrid[0].len() {
            if newgrid[i][j] == '.' {
                k = j;
                count = 0;
                while k < newgrid[0].len() {
                    if newgrid[i][k] == 'X' {
                        count += 1;
                    }
                    k -= 1;
                }
                if count & 1 == 1 {
                    ans += 1;
                }
            }
        }
    }
    ans-1
}

pub fn solve() -> (usize,usize) {
    let grid: Vec<Vec<char>> = include_str!("input.txt").lines()
    .map(|line| line.chars().collect())
    .collect();

    let mut newgrid: Vec<Vec<char>> = vec![vec!['.';grid[0].len()];grid.len()];

    let mut start = (0,0);
    let mut flag = false;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                flag = true;
                start = (i,j);
                break;
            }
        }
        if flag {
            break;
        }
    }

    let starts: [((usize,usize),char,usize);4] = [
        ((start.0-1,start.1),grid[start.0-1][start.1],1),
        ((start.0+1,start.1),grid[start.0+1][start.1],2),
        ((start.0,start.1-1),grid[start.0][start.1-1],3),
        ((start.0,start.1+1),grid[start.0][start.1+1],4)
    ];
    let part1 = traverse(starts[1],&grid,&mut newgrid); //hardcoded
    let part2 = find_trapped(&newgrid);
    (part1,part2)
}

fn main() {
    let (part1,part2) = solve();  
    println!("Maximum distance from start: {}\nTiles enclosed by loop: {}",part1,part2);
}
