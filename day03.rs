fn main() {

    let start = std::time::Instant::now();

    let grid: Vec<Vec<char>> = include_str!("input.txt").lines()
    .map(|line| line.chars().collect())
    .collect();

    let x = grid.len();
    let y = grid[0].len();

    let mut numbers = vec![vec![vec![]; y]; x];

    let mut parts = 0;
    let mut gear_ratios = 0;

    let is_symbol = |c: char| 
    !c.is_digit(10) && c != '.';

    let is_in_bounds = |i: isize, j: isize|
    0 <= i && i < x as isize 
    && 0 <= j && j < y as isize;

    for i in 0..x {
        let mut j = 0;
        while j < y {
            if grid[i][j].is_digit(10) {
                let mut part = 0;
                let startc = j;
                while j < y && grid[i][j].is_digit(10) {
                    part = part * 10 + (grid[i][j] as i32 - 48);
                    j += 1;
                }
                for boxr in (i as isize - 1)..=(i as isize + 1) {
                    for boxc in (startc as isize - 1)..=(j as isize) {
                        if is_in_bounds(boxr, boxc) 
                        && is_symbol(grid[boxr as usize][boxc as usize]) {
                            numbers[boxr as usize][boxc as usize].push(part);
                            parts+=part;
                            break;
                        }
                    }
                }
            } 
            j+=1;
        }
    }

    for i in 0..x {
        for j in 0..y {
            if grid[i][j]=='*' && numbers[i][j].len()==2 {
                gear_ratios += numbers[i][j][0] * numbers[i][j][1];
            }
        }
    }
    println!("Sum of parts: {}\nSum of gear ratios: {}\nTime taken: {:?}",
    parts,gear_ratios,start.elapsed());
}
