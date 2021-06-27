use std::option::Option;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut grid = construct_grid(6);

    mazeify(&mut grid);
}

fn construct_grid(size: u8) -> Vec<Vec<u8>> {
    let size: u16 = (size * 2 + 1) as u16;
    let mut grid: Vec<Vec<u8>> = vec![vec![0; size as _]; size as _];

    for idx in 0..(size as usize) {
        if idx % 2 == 0 {
            grid[idx] = vec![1; size as _];
        } else {
            for inner_idx in 0..(size as usize) {
                if inner_idx % 2 == 0 {
                    grid[idx][inner_idx] = 1;
                }
            }
        }
    }

    return grid;
}

fn get_unvisited_neighbours(cell: &[i8; 2], visited: &Vec<[i8; 2]>, max: i8) -> Option<Vec<[i8; 2]>> {
    let mut neighbours: Vec<[i8; 2]> = vec![];
    let surroundings: [[i8; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

    for [x, y] in surroundings.iter() {
        let neighbour = [cell[0] + x, cell[1] + y];
        if [-1, max].contains(&neighbour[0]) || [-1, max].contains(&neighbour[1]) {
            continue
        } else if !visited.contains(&neighbour) {
            neighbours.push(neighbour);
        }
    }

    println!("{:?}", neighbours);

    if neighbours.is_empty() {
        return None;
    } else {
        return Some(neighbours);
    }
}

fn mazeify(grid: &mut Vec<Vec<u8>>) -> &mut Vec<Vec<u8>> {
    let mut rng = thread_rng();
    let mut visited: Vec<[i8; 2]> = vec![[1, 1]];
    let mut stack: Vec<[i8; 2]> = vec![[1, 1]];

    while !stack.is_empty() {
        let mut current_cell = stack.pop().unwrap();
        let neighbour = match get_unvisited_neighbours(&current_cell, &visited, (grid.len() - 1) as i8) {
            Some(neighbours) => neighbours.choose(&mut rng).cloned().unwrap(),
            None => continue
        };

        println!("{:?}", neighbour);

    }

    return grid;
}