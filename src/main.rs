fn main() {
    let grid = construct_maze(3);

    // println!("{:?}", grid);
}

fn construct_maze(size: u8) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = vec![vec![0; size as _]; size as _];
    
    for row in grid {
        println!("{:?}", row);
    }



    return grid;
}