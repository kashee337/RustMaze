use super::gen_maze;
use super::types;

use std::collections::VecDeque;

pub struct Explorer {
    h: u32,
    w: u32,
    cx: u32,
    cy: u32,
    sx: u32,
    sy: u32,
    gx: u32,
    gy: u32,
    maze: Vec<String>,
    shortest_path: Vec<Vec<i32>>,
}

impl Explorer {
    pub fn new(mut maze: Vec<String>) -> Result<Explorer, String> {
        let h: u32 = maze.len() as u32;
        let w: u32 = maze[0].len() as u32;
        let mut sx: u32 = 0;
        let mut sy: u32 = 0;
        let mut gx: u32 = 0;
        let mut gy: u32 = 0;
        for i in 0..h {
            for j in 0..w {
                if maze[i as usize].chars().nth(j as usize).unwrap() == 'S' {
                    if sx > 0 || sy > 0 {
                        Err("invalid maze!".to_string())?;
                    }
                    sy = i as u32;
                    sx = j as u32;
                } else if maze[i as usize].chars().nth(j as usize).unwrap() == 'G' {
                    if gx > 0 || gy > 0 {
                        Err("invalid maze!".to_string())?;
                    }
                    gy = i as u32;
                    gx = j as u32;
                }
            }
        }
        if (sx == 0 && sy == 0) || (gx == 0 && gy == 0) {
            Err("invalid maze!".to_string())?;
        }
        Ok(Explorer {
            cx: sx,
            cy: sy,
            sx: sx,
            sy: sy,
            gx: gx,
            gy: gy,
            h: h,
            w: w,
            shortest_path: bfs(&mut maze, sx, sy, gx, gy, w, h).unwrap(),
            maze: maze,
        })
    }
    pub fn action(&mut self, direction: &types::Action) -> Result<(), String> {
        let mut nx = self.cx as i32;
        let mut ny = self.cy as i32;
        match direction {
            types::Action::Up => ny -= 1,
            types::Action::Down => ny += 1,
            types::Action::Right => nx += 1,
            types::Action::Left => nx -= 1,
            types::Action::Quit => (),
        }

        if let Ok(c) = gen_maze::bounds_check(&mut self.maze, nx, ny) {
            if c != '#' {
                self.cx = nx as u32;
                self.cy = ny as u32;
                Ok(())
            } else {
                Err("wall".to_string())
            }
        } else {
            Err("out of field".to_string())
        }
    }
    pub fn draw(&mut self) -> Vec<String> {
        let mut maze = self.maze.clone();
        maze[self.cy as usize].replace_range(self.cx as usize..(self.cx as usize + 1), "o");
        maze
    }
    pub fn check_status(&mut self) -> Result<u32, types::Action> {
        let dist: u32 = self.shortest_path[self.cy as usize][self.cx as usize] as u32;
        if dist == 0 {
            Err(types::Action::Quit)
        } else {
            Ok(dist)
        }
    }
}

fn bfs(
    _maze: &mut Vec<String>,
    sx: u32,
    sy: u32,
    gx: u32,
    gy: u32,
    w: u32,
    h: u32,
) -> Result<Vec<Vec<i32>>, String> {
    let mut q: VecDeque<((usize, usize), i32)> = VecDeque::new();
    q.push_front(((gx as usize, gy as usize), 0));
    let mut shortest_path = vec![vec![-1; w as usize]; h as usize];
    let dx: [i32; 4] = [-1, 0, 1, 0];
    let dy: [i32; 4] = [0, -1, 0, 1];
    while !q.is_empty() {
        let xy = q.pop_back().unwrap();
        shortest_path[xy.0 .1][xy.0 .0] = xy.1;
        for i in 0..4 {
            let nx = xy.0 .0 as i32 + dx[i];
            let ny = xy.0 .1 as i32 + dy[i];
            if let Ok(c) = gen_maze::bounds_check(_maze, nx, ny) {
                if shortest_path[ny as usize][nx as usize] < 0 && c != '#' {
                    q.push_back(((nx as usize, ny as usize), xy.1 + 1));
                }
            }
        }
    }
    if shortest_path[sy as usize][sx as usize] >= 0 {
        Ok(shortest_path)
    } else {
        Err("can't find shortest path".to_string())
    }
}
