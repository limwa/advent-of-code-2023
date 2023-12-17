use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Pipe {
    Horizontal,
    Vertical,
    SouthToEast,
    SouthToWest,
    NorthToEast,
    NorthToWest,
}

pub struct PipeMaze {
    pub width: i64,
    pub height: i64,

    pub pipes: HashMap<(i64, i64), Pipe>,
    pub start: (i64, i64),
}

pub fn read_pipe_maze(reader: impl Iterator<Item = String>) -> PipeMaze {
    
    let mut pipe_maze = PipeMaze {
        width: 0,
        height: 0,

        pipes: HashMap::new(),
        start: (0, 0),
    };

    reader.enumerate()
        .flat_map(|(y, line)| {
            pipe_maze.width = line.len() as i64;
            pipe_maze.height = y as i64 + 1;

            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    let location = (x as i64, y as i64);
                    let pipe_type = match c {
                        '-' => Pipe::Horizontal,
                        '|' => Pipe::Vertical,
                        'F' => Pipe::SouthToEast,
                        '7' => Pipe::SouthToWest,
                        'L' => Pipe::NorthToEast,
                        'J' => Pipe::NorthToWest,
                        'S' => return Some((location, None)),
                        _ => return None,
                    };

                    Some((location, Some(pipe_type)))
                })
                .collect::<Vec<_>>()
        })
        .for_each(|(location, pipe_type)| {
            if let Some(pipe_type) = pipe_type {
                pipe_maze.pipes.insert(location, pipe_type);
            } else {
                pipe_maze.start = location;
            }
        });

        // Find the type of pipe at the start location
        let (start_x, start_y) = pipe_maze.start;
        
        let mut connected_directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(|(dx, dy)| {
                let location = (start_x + dx, start_y + dy);
                match pipe_maze.pipes.get(&location) {
                    Some(pipe) => get_pipe_connections(pipe).iter()
                        .find(|(new_dx, new_dy)| *dx == -*new_dx && *dy == -*new_dy)
                        .map(|_| (*dx, *dy)),
                    None => None,
                }
            })
            .collect::<Vec<_>>();

        connected_directions.sort();
        
        let start_pipe = match connected_directions.as_slice() {
            [(-1, 0), (0, 1)] => Pipe::SouthToWest,
            [(0, 1), (1, 0)] => Pipe::SouthToEast,
            [(-1, 0), (0, -1)] => Pipe::NorthToWest,
            [(0, -1), (1, 0)] => Pipe::NorthToEast,
            [(-1, 0), (1, 0)] => Pipe::Horizontal,
            [(0, -1), (0, 1)] => Pipe::Vertical,
            _ => panic!("Invalid start pipe. Connected directions: {:?}", connected_directions),
        };

        pipe_maze.pipes.insert(pipe_maze.start, start_pipe);

        pipe_maze
}

pub fn get_pipe_connections(pipe: &Pipe) -> Vec<(i64, i64)> {
    match pipe {
        Pipe::Horizontal => vec![(1, 0), (-1, 0)],
        Pipe::Vertical => vec![(0, 1), (0, -1)],
        Pipe::SouthToEast => vec![(0, 1), (1, 0)],
        Pipe::SouthToWest => vec![(0, 1), (-1, 0)],
        Pipe::NorthToEast => vec![(0, -1), (1, 0)],
        Pipe::NorthToWest => vec![(0, -1), (-1, 0)],
    }
}