use std::collections::HashSet;

fn find_energized_tiles(x: usize, y: usize, dir: usize, grid: &Vec<Vec<char>>) -> usize {
    let mut cache: HashSet<(usize,usize,usize)> = HashSet::new();
    let mut energized_tiles: HashSet<(usize,usize)> = HashSet::new();
    beam(x,y,dir,grid,&mut cache);    
    for &x in cache.iter() {
        energized_tiles.insert((x.0,x.1));
    }
    energized_tiles.len()
}

fn solve(grid: &Vec<Vec<char>>,part2: bool) -> usize {
    if !part2 {
        return find_energized_tiles(0,0,4,grid);
    }
    let (p,q) = (grid.len(),grid[0].len());

    let left = (0..p)
    .map(|a| find_energized_tiles(a,0,4,grid))
    .max().unwrap();

    let right = (0..p)
    .map(|a| find_energized_tiles(a,q-1,3,grid))
    .max().unwrap();

    let top = (0..q)
    .map(|a| find_energized_tiles(0,a,2,grid))
    .max().unwrap();

    let bottom = (0..q)
    .map(|a| find_energized_tiles(p-1,a,1,grid))
    .max().unwrap();

    *[left,right,top,bottom].iter().max().unwrap()
}

fn beam(mut x: usize, mut y:usize, mut dir:usize, grid: &Vec<Vec<char>>, cache: &mut HashSet<(usize,usize,usize)>) {
    while x < grid.len() && y < grid[0].len() {

        if cache.contains(&(x,y,dir)) {
            break;
        }
        cache.insert((x,y,dir));

        match (dir,grid[x][y]) {
            // 1 up 2 down 3 left 4 right
            (1,'.') => x -= 1, 
            (2,'.') => x += 1,
            (3,'.') => y -= 1,
            (4,'.') => y += 1,
    
            (1,'/') => {y += 1; dir = 4},
            (2,'/') => {y -= 1; dir = 3},
            (3,'/') => {x += 1; dir = 2},
            (4,'/') => {x -= 1; dir = 1},
    
            (1,'\\') => {y -= 1; dir = 3},
            (2,'\\') => {y += 1; dir = 4},
            (3,'\\') => {x -= 1; dir = 1},
            (4,'\\') => {x += 1; dir = 2},
    
            (1,'|') => x -= 1,
            (2,'|') => x += 1,
            (3,'|') => { 
                beam(x-1,y,1,grid,cache);
                beam(x+1,y,2,grid,cache);
            },
            (4,'|') => {                                    
                beam(x-1,y,1,grid,cache);
                beam(x+1,y,2,grid,cache);
            },
    
            (1,'-') => {
                beam(x,y-1,3,grid,cache);
                beam(x,y+1,4,grid,cache);
            },
            (2,'-') => {
                beam(x,y-1,3,grid,cache);
                beam(x,y+1,4,grid,cache);
            },
            (3,'-') => y -= 1,
            (4,'-') => y += 1,
    
               _    => break,
        }
    }
}

fn main() {
    let grid: Vec<Vec<char>> = include_str!("input.txt").lines()
    .map(|line| line.chars().collect())
    .collect();
    let (part1,part2) = (solve(&grid,false),solve(&grid,true));
    println!("Part 1: {}\nPart 2: {}",part1,part2);
}
