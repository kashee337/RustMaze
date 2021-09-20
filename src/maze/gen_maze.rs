use super::types;
use rand::Rng;

pub fn generate(w: u32, h: u32) -> Result<Vec<String>, String> {
    if w < 5 || h < 5 {
        Err("w and h must be greater than 5".to_string())?;
    }
    let _w = w + ((w % 2 == 0) as u32);
    let _h = h + ((h % 2 == 0) as u32);
    let mut maze = Vec::new();
    let mut anchor = Vec::new();
    for y in 0.._h {
        let mut row = "".to_string();
        for x in 0.._w {
            if y == 0 || y == _h - 1 || x == 0 || x == _w - 1 {
                row = row + "#";
            } else {
                row = row + ".";
                if y % 2 == 0 && x % 2 == 0 {
                    anchor.push((y as usize, x as usize));
                }
            }
        }
        maze.push(row);
    }

    let mut rng = rand::thread_rng();
    let mut current_walls = Vec::new();
    while !anchor.is_empty() {
        let index = rng.gen_range(0..anchor.len()) as usize;
        let y = anchor[index].0 as i32;
        let x = anchor[index].1 as i32;
        anchor.remove(index);
        if maze[y as usize].chars().nth(x as usize).unwrap() != '#' {
            current_walls.clear();
            extend_wall(&mut maze, &mut current_walls, x, y);
        }
    }
    set_maze(&mut maze, 1, 0, "S").unwrap();
    set_maze(&mut maze, (_w - 2) as i32, (_h - 1) as i32, "G").unwrap();
    Ok(maze)
}

pub fn bounds_check(maze: &mut Vec<String>, x: i32, y: i32) -> Result<char, String> {
    let h = maze.len() as usize;
    let w = maze[0].len() as usize;
    if x < 0 || x >= w as i32 || y < 0 || y >= h as i32 {
        Err("out of field".to_string())?;
    }
    Ok(maze[y as usize].chars().nth(x as usize).unwrap())
}

fn extend_wall(maze: &mut Vec<String>, current_walls: &mut Vec<(usize, usize)>, x: i32, y: i32) {
    let mut directions = Vec::new();
    if let Ok(_) = bounds_check(maze, x, y - 1) {
        if maze[(y - 1) as usize].chars().nth(x as usize).unwrap() == '.'
            && !current_walls.contains(&((y - 2) as usize, x as usize))
        {
            directions.push(types::Action::Up);
        }
    }
    if let Ok(_) = bounds_check(maze, x, y + 1) {
        if maze[(y + 1) as usize].chars().nth(x as usize).unwrap() == '.'
            && !current_walls.contains(&((y + 2) as usize, x as usize))
        {
            directions.push(types::Action::Down);
        }
    }
    if let Ok(_) = bounds_check(maze, x - 1, y) {
        if maze[y as usize].chars().nth((x - 1) as usize).unwrap() == '.'
            && !current_walls.contains(&(y as usize, (x - 2) as usize))
        {
            directions.push(types::Action::Left);
        }
    }
    if let Ok(_) = bounds_check(maze, x + 1, y) {
        if maze[y as usize].chars().nth((x + 1) as usize).unwrap() == '.'
            && !current_walls.contains(&(y as usize, (x + 2) as usize))
        {
            directions.push(types::Action::Right);
        }
    }

    // let mut rng = rand::thread_rng();
    if !directions.is_empty() {
        set_wall(maze, current_walls, x, y).unwrap();
        let v = directions.len();
        let index = rand::thread_rng().gen_range(0..directions.len() as usize);
        let mut nx = x;
        let mut ny = y;
        let mut is_path = false;
        match directions[index] {
            types::Action::Up => {
                ny -= 2;
                is_path = maze[ny as usize].chars().nth(nx as usize).unwrap() == '.';
                set_wall(maze, current_walls, x, y - 1).unwrap();
                set_wall(maze, current_walls, x, y - 2).unwrap();
            }
            types::Action::Down => {
                ny += 2;
                is_path = maze[ny as usize].chars().nth(nx as usize).unwrap() == '.';
                set_wall(maze, current_walls, x, y + 1).unwrap();
                set_wall(maze, current_walls, x, y + 2).unwrap();
            }
            types::Action::Right => {
                nx += 2;
                is_path = maze[ny as usize].chars().nth(nx as usize).unwrap() == '.';
                set_wall(maze, current_walls, x + 1, y).unwrap();
                set_wall(maze, current_walls, x + 2, y).unwrap();
            }
            types::Action::Left => {
                nx -= 2;
                is_path = maze[ny as usize].chars().nth(nx as usize).unwrap() == '.';
                set_wall(maze, current_walls, x - 1, y).unwrap();
                set_wall(maze, current_walls, x - 2, y).unwrap();
            }
            _ => (),
        }
        if is_path {
            extend_wall(maze, current_walls, nx, ny);
        }
    } else {
        if let Some(xy) = current_walls.pop() {
            extend_wall(maze, current_walls, xy.1 as i32, xy.0 as i32);
        }
    }
}

fn set_wall(
    maze: &mut Vec<String>,
    current_walls: &mut Vec<(usize, usize)>,
    x: i32,
    y: i32,
) -> Result<(), String> {
    if x % 2 == 0 && y % 2 == 0 {
        current_walls.push((y as usize, x as usize));
    }
    set_maze(maze, x, y, "#")
}

fn set_maze(maze: &mut Vec<String>, x: i32, y: i32, c: &str) -> Result<(), String> {
    bounds_check(maze, x, y)?;
    maze[y as usize].replace_range((x as usize)..(x + 1) as usize, c);
    Ok(())
}
