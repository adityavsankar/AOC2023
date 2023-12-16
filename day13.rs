fn smudge_reflections(mut grid: Vec<Vec<char>>, col: bool) -> usize {
    let mut errors;
    if col { //transpose
        grid = (0..grid[0].len())
        .map(|col| {
            (0..grid.len())
            .map(|row| grid[row][col])
            .collect()
        })
        .collect();
    }
    'a:for i in 1..grid.len() {
        errors = 0;
        let a = &grid[i-1];
        let b = &grid[i];
        for (x,y) in a.iter().zip(b) {
            if *x != *y {
                errors += 1;
                if errors > 1 {
                    continue 'a;
                }
            }
        }
        for k in 1..=(i-1).min(grid.len()-i-1) {
            let c = &grid[i-k-1];
            let d = &grid[i+k];
            for (x,y) in c.iter().zip(d) {
                if *x != *y {
                    errors += 1;
                    if errors > 1 {
                        continue 'a;
                    }
                }
            }
        }
        if errors == 1 {
            return i;
        }
    }
    0
}

fn reflections(mut grid: Vec<Vec<char>>, col: bool) -> usize {
    if col {
        grid = (0..grid[0].len())
        .map(|col| {
            (0..grid.len())
            .map(|row| grid[row][col])
            .collect()
        })
        .collect();
    }
    'a:for i in 1..grid.len() {
        if grid[i-1] == grid[i] {
            for k in 1..=(i-1).min(grid.len()-i-1) {
                if grid[i-k-1] != grid[i+k] {
                    continue 'a;
                }
            }
            return i;
        }
    }
    0
}

fn solve() -> (usize,usize) {
    let input = include_str!("input.txt").lines();
    let mut grid: Vec<Vec<char>> = vec![];
    let mut grids: Vec<Vec<Vec<char>>> = vec![];
    for line in input {
        if line.is_empty() {
            grids.push(grid);
            grid = vec![];
        }
        else {
            grid.push(line.chars().collect());
        }
    }

    let part1: usize = grids
    .iter()
    .map(|grid|
    (reflections(grid.clone(),false)*100) + reflections(grid.clone(),true))
    .sum();

    let part2: usize = grids
    .iter()
    .map(|grid|
    (smudge_reflections(grid.clone(),false)*100) + smudge_reflections(grid.clone(),true))
    .sum();

    (part1,part2)
}
fn main() {
    let (part1,part2) = solve();
    println!("Part 1: {}\nPart 2: {}",part1,part2);
}
