mod maze {
    pub mod explorer;
    pub mod gen_maze;
    pub mod types;
}
use clap::{App, Arg};
use std::io::{stdin, stdout, Write};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::spawn;
use std::time::Instant;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::*;
struct Message {
    _action: maze::types::Action,
}

fn parser_key(
    c: Result<termion::event::Key, std::io::Error>,
) -> Result<maze::types::Action, String> {
    match c {
        Ok(event::Key::Up) => Ok(maze::types::Action::Up),
        Ok(event::Key::Down) => Ok(maze::types::Action::Down),
        Ok(event::Key::Right) => Ok(maze::types::Action::Right),
        Ok(event::Key::Left) => Ok(maze::types::Action::Left),
        Ok(event::Key::Ctrl('c')) => Ok(maze::types::Action::Quit),
        _ => Err("invalid key".to_string()),
    }
}

fn main() {
    let args = App::new("Rust Maze")
        .version("0.1.0")
        .author("kashee337")
        .arg(
            Arg::new("height")
                .about("height of maze")
                .short('h')
                .long("height")
                .default_value("11"),
        )
        .arg(
            Arg::new("width")
                .about("width of maze")
                .short('w')
                .long("width")
                .default_value("31"),
        )
        .get_matches();

    // parse args
    let w: u32 = args.value_of("width").unwrap().parse().unwrap();
    let h: u32 = args.value_of("height").unwrap().parse().unwrap();
    // sender,reciever
    let (tx, rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    // key bind
    let _handle_receive = spawn(move || {
        for c in stdin().keys() {
            if let Ok(key) = parser_key(c) {
                let message = Message { _action: key };
                tx.send(message).unwrap();
            } else {
            }
        }
    });

    // initialize maze
    let maze = maze::gen_maze::generate(w, h).unwrap();
    let mut explorer = maze::explorer::Explorer::new(maze).unwrap();
    let st = Instant::now();
    // initial draw
    let crr_maze = explorer.draw();

    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", clear::All).unwrap();
    for (i, row) in crr_maze.iter().enumerate() {
        write!(stdout, "{}", cursor::Goto(1, i as u16 + 1)).unwrap();
        writeln!(stdout, "{}", row).unwrap();
    }
    write!(stdout, "{}", cursor::Goto(1, crr_maze.len() as u16 + 1)).unwrap();
    writeln!(stdout, "Go!",).unwrap();

    loop {
        for message in rx.iter() {
            let mut _action = message._action;
            // take action
            let _ = explorer.action(&_action);
            // update maze
            let crr_maze = explorer.draw();
            let status = explorer.check_status();

            // draw current status
            write!(stdout, "{}", clear::All).unwrap();
            for (i, row) in crr_maze.iter().enumerate() {
                write!(stdout, "{}", cursor::Goto(1, i as u16 + 1)).unwrap();
                writeln!(stdout, "{}", row).unwrap();
            }

            write!(stdout, "{}", cursor::Goto(1, crr_maze.len() as u16 + 1)).unwrap();
            if let Ok(dist) = status {
                writeln!(stdout, "distance:{}", dist).unwrap();
            } else if let Err(c) = status {
                writeln!(stdout, "Congratulations!").unwrap();
                write!(stdout, "{}", cursor::Goto(1, crr_maze.len() as u16 + 2)).unwrap();
                let end = st.elapsed();
                write!(
                    stdout,
                    "record:{}.{:03}sec",
                    end.as_secs(),
                    end.subsec_nanos() / 1_000_000
                )
                .unwrap();
                _action = c;
            }

            if let maze::types::Action::Quit = _action {
                std::process::exit(0)
            }
        }
        stdout.flush().unwrap();
    }
}
