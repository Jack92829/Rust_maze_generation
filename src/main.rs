use std::option::Option;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::stdin;

fn main() {
    loop {
        let mut num = String::new();

        println!("Enter a maze size below!");
        stdin().read_line(&mut num).expect("Hey, that inputs bad!");

        let num: u8 = match num.trim().parse() {
            Ok(value) => value,
            Err(bad_value) => {
                println!("{} is not a valid number", bad_value);
                continue
            }
        };

        let mut grid = construct_grid(num);

        for row in mazeify(&mut grid).into_iter() {
            for column in row {
                if *column == 0 {
                    print!("⬜");
                } else {
                    print!("⬛");
                }
            }
            println!("");
        }
    }
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

    for row in &grid {
        println!("{:?}", row);
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

    if neighbours.is_empty() {
        return None;
    } else {
        return Some(neighbours);
    }
}

fn remove_wall(current_cell: &[i8; 2], neighbour: &[i8; 2], grid: &mut Vec<Vec<u8>>) {
    let current_coords = [2 * current_cell[0] + 1, 2 * current_cell[1] + 1];
    let neighbour_coords = [2 * neighbour[0] + 1, 2 * neighbour[1] + 1];

    let x_diff = (current_coords[0] - neighbour_coords[0]) / 2;
    let y_diff = (current_coords[1] - neighbour_coords[1]) / 2;

    let wall_coords = [neighbour_coords[0] + x_diff, neighbour_coords[1] + y_diff];

    grid[wall_coords[1] as usize][wall_coords[0] as usize] = 0
}

fn mazeify(grid: &mut Vec<Vec<u8>>) -> &mut Vec<Vec<u8>> {
    let mut rng = thread_rng();
    let mut visited: Vec<[i8; 2]> = vec![[0, 0]];
    let mut stack: Vec<[i8; 2]> = vec![[0, 0]];

    while !stack.is_empty() {
        let current_cell = stack.pop().unwrap();
        let neighbour = match get_unvisited_neighbours(&current_cell, &visited, ((grid.len() - 1) / 2) as i8) {
            Some(neighbours) => {
                stack.push(current_cell);
                neighbours.choose(&mut rng).cloned().unwrap()
            },
            None => continue
        };

        remove_wall(&current_cell, &neighbour, grid);

        stack.push(neighbour);
        visited.push(neighbour);
    }
    return grid;
}