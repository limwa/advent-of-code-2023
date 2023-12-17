use std::{fs, io::{self, BufRead}};
use pipe_maze::*;

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let pipe_maze = read_pipe_maze(lines);
    println!("Pipe maze: {:?}", pipe_maze.pipes);

    let mut starting_location: Option<(i64, i64)> = None;

    for (offset_x, offset_y) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let location = (pipe_maze.start.0 + offset_x, pipe_maze.start.1 + offset_y);
        if let Some(pipe) = pipe_maze.pipes.get(&location) {
            let connections = get_pipe_connections(pipe);
            if connections.contains(&(-offset_x, -offset_y)) {
                starting_location = Some(location);
                break;
            }

        }
    }

    let mut current_location = starting_location.expect("No starting location found");
    let mut previous_location = pipe_maze.start;

    let mut path = vec![previous_location];

    while current_location != pipe_maze.start {
        path.push(current_location);
        
        let pipe = pipe_maze.pipes.get(&current_location).expect("No pipe found at location");
        let connections = get_pipe_connections(pipe);
        
        let next_location = connections.iter()
            .map(|(offset_x, offset_y)| (current_location.0 + offset_x, current_location.1 + offset_y))
            .find(|location| previous_location != *location)
            .expect("No next location found");
    
        previous_location = current_location;
        current_location = next_location;
    }

    println!("Path: {:?}", path);
    println!("Farthest distance: {}", path.len() / 2);

    Ok(())
}