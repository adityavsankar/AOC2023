use std::cmp::{min,max};

pub fn solve(n: usize) -> usize {
    let grid: Vec<Vec<char>> = include_str!("input.txt").lines()
    .map(|line| line.chars().collect())
    .collect();

    let tgrid: Vec<Vec<char>> = (0..grid[0].len())
    .map(|col| {
        (0..grid.len())
        .map(|row| grid[row][col])
        .collect()
    })
    .collect();

    let mut stars: Vec<(usize,usize)> = vec![];
    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    for (j,y) in tgrid.iter().enumerate() {
        if y.iter().all(|x| *x == '.') {
            empty_cols.push(j);
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                stars.push((i,j));
            }          
        }
        if grid[i].iter().all(|x| *x == '.') {
            empty_rows.push(i);
        }
    }

    let mut c = 0;
    let mut m = 0;

    for i in 0..stars.len() {
        for j in i+1..stars.len() {
            let (x1,y1) = (stars[i].0,stars[i].1);
            let (x2,y2) = (stars[j].0,stars[j].1);
            let dr = empty_rows.iter()
            .filter(|x| (min(x1,x2)..max(x1,x2)).contains(*x))
            .count();
            let dc = empty_cols.iter()
            .filter(|y| (min(y1,y2)..max(y1,y2)).contains(*y))
            .count();
            m += dr+dc;
            c += (x1).abs_diff(x2)+(y1).abs_diff(y2);
        }
    }
    c + (n-1) * m
}

fn main() {
    println!("Part 1: {}\nPart 2: {}",solve(2),solve(1000000));
}
