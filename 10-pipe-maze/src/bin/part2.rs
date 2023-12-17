use std::{fs, io::{self, BufRead}, collections::HashSet};
use pipe_maze::*;

fn find_pipe_path(pipe_maze: &PipeMaze) -> Vec<(i64, i64)> {
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

    path
}

#[derive(Debug, Clone)]
enum State  {
    OutsidePath,
    InsidePath,
    WaitingFrom(Pipe, Box<State>),
}

fn main() -> io::Result<()> {
    let file = fs::File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines()
        .filter_map(|line| line.ok());

    let pipe_maze = read_pipe_maze(lines);
    let path = find_pipe_path(&pipe_maze);

    println!("Path: {:?}", path);
    println!("Pipe Maze: {:?}", pipe_maze.pipes);

    let path_cells = path.iter().collect::<HashSet<_>>();

    let mut inside_count = 0;    

    for y in 0..pipe_maze.height {        
        let mut state = State::OutsidePath;

        for x in 0..pipe_maze.width {
            let location = (x, y);
            let is_path = path_cells.contains(&location);

            if is_path {
                let pipe = *pipe_maze.pipes.get(&location).unwrap();

                match pipe {
                    Pipe::NorthToEast | Pipe::SouthToEast => state = State::WaitingFrom(pipe, Box::new(state)),
                    Pipe::Vertical => {
                        state = match state {
                            State::InsidePath => State::OutsidePath,
                            State::OutsidePath => State::InsidePath,
                            _ => panic!("Invalid state. Current state: {:?}", state),
                        };
                    },
                    Pipe::NorthToWest => {
                        state = match state {
                            State::WaitingFrom(Pipe::SouthToEast, ref prev_state) => {
                                match prev_state.as_ref() {
                                    State::InsidePath => State::OutsidePath,
                                    State::OutsidePath => State::InsidePath,
                                    _ => panic!("Invalid state. Current state: {:?}", state),
                                }
                            },
                            State::WaitingFrom(Pipe::NorthToEast, prev_state) => *prev_state.clone(),
                            _ => panic!("Invalid state. Current state: {:?}", state),
                        };
                    },
                    Pipe::SouthToWest => {
                        state = match state {
                            State::WaitingFrom(Pipe::NorthToEast, ref prev_state) => {
                                match prev_state.as_ref() {
                                    State::InsidePath => State::OutsidePath,
                                    State::OutsidePath => State::InsidePath,
                                    _ => panic!("Invalid state. Current state: {:?}", state),
                                }
                            },
                            State::WaitingFrom(Pipe::SouthToEast, prev_state) => *prev_state.clone(),
                            _ => panic!("Invalid state. Current state: {:?}", state),
                        };
                    }
                    Pipe::Horizontal => {}
                }

                print!("=");
                continue;
            }

            if let State::InsidePath = state {
                inside_count += 1;

                print!("#");
            } else {
                print!(".");
            }
        }

        print!("\n");
    }
    
    print!("\n({})", inside_count);
    Ok(())
}


// ...........
// .S       7.
// .|F     7|.
// .||.....||.
// .||.....||.
// .|L 7.F J|.
// .|II|.|II|.
// .L  J.L  J.
// ...........