#![feature(slice_patterns)]
#![feature(libc)]

extern crate libc;

use libc::funcs::posix88::unistd::isatty;
use libc::consts::os::posix88::STDOUT_FILENO;
use std::io::{BufRead, stdin};

type Grid = Vec<Vec<Option<bool>>>;

fn read_grid() -> Grid {
    let stdin = stdin();
    let grid = stdin.lock().lines()
        .map(|line| {
            line.unwrap().chars()
                .map(|c| match c {
                    '0' => Some(false),
                    '1' => Some(true),
                    '.' => None,
                    c => { panic!("unexpected character '{}'", c); }
                }).collect()
        }).collect();
    check_size_grid(&grid);
    grid
}

fn check_size_grid(grid: &Grid) {
    let size = grid.len();
    if size % 2 == 1 {
        panic!("odd number of rows");
    }
    for (i, row) in grid.iter().enumerate() {
        if row.len() != size {
            panic!("line {}: not a square", i + 1);
        }
    }
}

fn print_grid(grid0: &Grid, grid: &Grid) {
    let isatty = match unsafe { isatty(STDOUT_FILENO) } {
        0 => false,
        1 => true,
        _ => { panic!("invalid return value: isatty()"); }
    };
    let mut buffer = String::with_capacity(grid.len() * (grid.len() * 10 + 1));
    for (row0, row) in grid0.iter().zip(grid.iter()) {
        for (tile0, tile) in row0.iter().zip(row.iter()) {
            match *tile {
                Some(true) => {
                    if tile == tile0 || !isatty { buffer.push('1'); }
                    else { buffer.extend("\u{1b}[31m1\u{1b}[0m".chars()); }
                },
                Some(false) => {
                    if tile == tile0 || !isatty { buffer.push('0'); }
                    else { buffer.extend("\u{1b}[34m0\u{1b}[0m".chars()); }
                },
                None => { buffer.push('.'); }
            }
        }
        buffer.push('\n');
    }
    print!("{}", buffer);
}

fn apply_rule1(grid: &mut Grid) -> bool {
    // You can't put more than two identical numbers next to each other in a line
    let mut rule_applied = false;
    for i in 0..grid.len() {
        for j in 0..grid.len() - 2 {
            { // horizontal
                let trio = &mut grid[i][j..j + 3];
                match trio {
                    [None, Some(a), Some(b)] if a == b => { trio[0] = Some(!a); rule_applied = true; },
                    [Some(a), None, Some(b)] if a == b => { trio[1] = Some(!a); rule_applied = true; },
                    [Some(a), Some(b), None] if a == b => { trio[2] = Some(!a); rule_applied = true; },
                    _ => {},
                }
            }
            { // vertical
                let trio = [grid[j][i], grid[j + 1][i], grid[j + 2][i]];
                match trio {
                    [None, Some(a), Some(b)] if a == b => { grid[j    ][i] = Some(!a); rule_applied = true; },
                    [Some(a), None, Some(b)] if a == b => { grid[j + 1][i] = Some(!a); rule_applied = true; },
                    [Some(a), Some(b), None] if a == b => { grid[j + 2][i] = Some(!a); rule_applied = true; },
                    _ => {},
                }
            }
        }
    }
    rule_applied
}

fn apply_rule2(grid: &mut Grid) -> bool {
    // The number of 1s and 0s on each row and column must match
    let mut rule_applied = false;
    for i in 0..grid.len() {
        let (mut h, mut v) = ([0; 2], [0; 2]);
        for j in 0..grid.len() {
            if let Some(a) = grid[i][j] { if a { h[1] += 1; } else { h[0] += 1; } }
            if let Some(a) = grid[j][i] { if a { v[1] += 1; } else { v[0] += 1; } }
        }
        if h[0] == grid.len() / 2 && h[1] != h[0] {
            rule_applied = true; 
            for j in 0..grid.len() {
                if grid[i][j] == None { grid[i][j] = Some(true); }
            }
        }
        else if h[1] == grid.len() / 2 && h[0] != h[1] {
            rule_applied = true; 
            for j in 0..grid.len() {
                if grid[i][j] == None { grid[i][j] = Some(false); }
            }
        }
        if v[0] == grid.len() / 2 && v[1] != v[0] {
            rule_applied = true; 
            for j in 0..grid.len() {
                if grid[j][i] == None { grid[j][i] = Some(true); }
            }
        }
        else if v[1] == grid.len() / 2 && v[0] != v[1] {
            rule_applied = true; 
            for j in 0..grid.len() {
                if grid[j][i] == None { grid[j][i] = Some(false); }
            }
        }
    }
    rule_applied
}

fn apply_rule3(grid: &mut Grid) -> bool {
    // You can't have two identical rows or two identical columns
    let mut rule_applied = false;
    for i in 0..grid.len() {
        if grid[i].iter().filter(|&&value| value == None).count() == 2 {
            for j in 0..grid.len() {
                if j != i
                    && !grid[j].contains(&None)
                    && grid[i].iter().zip(grid[j].iter()).filter(|&(&value, _)| value != None).all(|(value, other)| value == other) {
                        for k in 0..grid.len() {
                            if grid[i][k] == None {
                                grid[i][k] = Some(!grid[j][k].unwrap());
                            }
                        }
                        rule_applied = true;
                        break
                    }
            }
        }
        if (0..grid.len()).map(|j| grid[j][i] == None).filter(|&b| b).count() == 2 {
            for j in 0..grid.len() {
                if j != i
                    && (0..grid.len()).all(|k| grid[k][j] != None)
                    && (0..grid.len()).filter(|&k| grid[k][i] != None).all(|k| grid[k][i] == grid[k][j]) {
                        for k in 0..grid.len() {
                            if grid[k][i] == None {
                                grid[k][i] = Some(!grid[k][j].unwrap());
                            }
                        }
                        rule_applied = true;
                        break
                    }
            }
        }
    }
    rule_applied
}

fn solve_grid(grid: &mut Grid) {
    while apply_rule1(grid)
        || apply_rule2(grid)
        || apply_rule3(grid) { }
}

fn main() {
    let grid0 = read_grid();
    let mut grid = grid0.clone();
    solve_grid(&mut grid);
    print_grid(&grid0, &grid);
}