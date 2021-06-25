
fn main() {
    let grid = construct_maze(6);

    for row in grid {
        println!("{:?}", row);
    }
}

fn construct_maze(size: u8) -> Vec<Vec<u8>> {
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