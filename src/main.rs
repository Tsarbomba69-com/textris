use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::SetSize,
};
use rand::Rng;

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGTH: usize = 30;
const FIELD_WIDTH: usize = 12;
const FIELD_HEIGTH: usize = 18;
// Tetronimos 4x4
const TETROMINO: &'static [&'static str] = &[
    "..X...X...X...X.",
    "..X..XX...X.....",
    ".....XX..XX.....",
    "..X..XX..X......",
    ".X...XX...X.....",
    ".X...X...XX.....",
    "..X...X..XX.....",
];

fn rotate(px: i32, py: i32, r: usize) -> usize {
    match r % 4 {
        0 => (py * 4 + px) as usize,
        1 => (12 + py - px * 4) as usize,
        2 => (15 - py * 4 - px) as usize,
        3 => (3 - py + px * 4) as usize,
        _ => 0,
    }
}

fn can_move(piece: u8, orientation: usize, pos_x: i32, pos_y: i32, player_field: &[usize]) -> bool {
    for px in 0..4 as i32 {
        for py in 0..4 as i32 {
            // Get index into piece
            let player_idx = rotate(px, py, orientation);

            // Get index into field
            let field_idx = (pos_y + py) * FIELD_WIDTH as i32 + pos_x + px;

            if pos_x + px >= 0 && pos_x + px < FIELD_WIDTH as i32 {
                if pos_y + py >= 0 && pos_y + py < FIELD_HEIGTH as i32 {
                    // In Bounds so do collision check
                    if TETROMINO[piece as usize].chars().nth(player_idx).unwrap() != '.'
                        && player_field[field_idx as usize] != 0
                    {
                        return false; // fail on first hit
                    }
                }
            }
        }
    }

    true
}

fn render_line(lines: &mut Vec<usize>, buffer: &String, player_field: &mut [usize]) -> () {
    // Draw line completion
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::DarkBlue),
        Print(buffer),
        SetSize(SCREEN_WIDTH as u16, SCREEN_HEIGTH as u16)
    )
    .unwrap();
    std::thread::sleep(Duration::from_millis(60)); // Delay a bit

    for l in lines {
        for px in 1..FIELD_WIDTH - 1 {
            let mut py = *l;
            while py > 0 {
                player_field[py as usize * FIELD_WIDTH + px] =
                    player_field[(py as usize - 1) * FIELD_WIDTH + px];
                py -= 1;
            }
            player_field[px] = 0;
        }
    }

    print!("{esc}c", esc = 27 as char); //Clear terminal windows
}

fn main() {
    let mut player_field: Vec<usize> = vec![0usize; FIELD_WIDTH * FIELD_HEIGTH]; // Create play field buffer

    for x in 0..FIELD_WIDTH {
        // Board Boundary
        for y in 0..FIELD_HEIGTH {
            player_field[y * FIELD_WIDTH + x] =
                if x == 0 || x == FIELD_WIDTH - 1 || y == FIELD_HEIGTH - 1 {
                    9
                } else {
                    0
                };
        }
    }

    let mut pos_x: i32 = FIELD_WIDTH as i32 / 2;
    let mut pos_y: i32 = 0;
    let mut orientation = 0;
    let mut piece: u8 = rand::thread_rng().gen_range(0..7);
    // let mut piece: u8 = 2; // For debugging
    let mut is_falling: bool = false;
    let mut game_over = false;
    let speed = 20;
    let mut speed_count = 0;
    let mut lines: Vec<usize> = Vec::new();

    while !game_over {
        // Timing =======================
        std::thread::sleep(Duration::from_millis(10)); // Small Step = 1 Game Tick
        speed_count += 1;
        is_falling = speed_count == speed;

        // Create Screen Buffer
        let mut screen: Vec<char> = vec![' '; SCREEN_WIDTH * SCREEN_HEIGTH];

        // Game Logic =====================
        if poll(Duration::from_millis(10)).unwrap() {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Char('d') => {
                        pos_x +=
                            can_move(piece, orientation, pos_x + 1, pos_y, &player_field) as i32
                    }
                    KeyCode::Char('a') => {
                        pos_x -=
                            can_move(piece, orientation, pos_x - 1, pos_y, &player_field) as i32
                    }
                    KeyCode::Char('s') => {
                        pos_y +=
                            can_move(piece, orientation, pos_x, pos_y + 1, &player_field) as i32
                    }
                    KeyCode::Char('r') => {
                        orientation +=
                            can_move(piece, orientation + 1, pos_x, pos_y, &player_field) as usize
                    }
                    _ => (),
                },
                Event::Mouse(_event) => (),
                Event::Resize(_width, _height) => (),
            }
        } else {
            // Timeout expired and no `Event` is available
        }
        // Force the piece down the playfield if it's time
        if is_falling {
            speed_count = 0;
            if can_move(piece, orientation, pos_x, pos_y + 1, &player_field[..]) {
                pos_y += 1;
                print!("{esc}c", esc = 27 as char); //Clear terminal windows
            } else {
                for px in 0..4 {
                    for py in 0..4 {
                        if TETROMINO[piece as usize]
                            .chars()
                            .nth(rotate(px, py, orientation))
                            .unwrap()
                            != '.'
                        {
                            player_field[(pos_y + py) as usize * FIELD_WIDTH
                                + pos_x as usize
                                + px as usize] = (piece + 1) as usize;
                        }
                    }
                }

                // Check for lines
                for py in 0..4 as usize {
                    if pos_y as usize + py < FIELD_HEIGTH - 1 {
                        let mut line = true;
                        for px in 1..FIELD_WIDTH - 1 {
                            line &= player_field[(pos_y as usize + py) * FIELD_WIDTH + px] != 0;
                        }

                        if line {
                            // Remove Line, set to =
                            for px in 1..FIELD_WIDTH - 1 {
                                player_field[(pos_y as usize + py) * FIELD_WIDTH + px] = 8;
                            }
                            lines.push(pos_y as usize + py);
                        }
                    }
                }

                pos_x = FIELD_WIDTH as i32 / 2;
                pos_y = 0;
                orientation = 0;
                piece = rand::thread_rng().gen_range(0..7);
                // piece = 2; // For debbuging
            }
        }

        // Draw Field
        for x in 0..FIELD_WIDTH {
            for y in 0..FIELD_HEIGTH {
                screen[(y + 2) * SCREEN_WIDTH + x + 2] = {
                    " ABCDEFG=#"
                        .chars()
                        .nth(player_field[y * FIELD_WIDTH + x])
                        .unwrap()
                }
            }
        }

        // Draw Current Piece
        for px in 0..4 {
            for py in 0..4 {
                if TETROMINO[piece as usize]
                    .chars()
                    .nth(rotate(px, py, orientation))
                    .unwrap()
                    != '.'
                {
                    screen[(pos_y + py + 2) as usize * SCREEN_WIDTH + (pos_x + px + 2) as usize] =
                        (piece + 65) as char;
                }
            }
        }

        let buffer: String = screen.into_iter().collect();

        // Animate Line Completion
        if !lines.is_empty() {
            render_line(&mut lines, &buffer, &mut player_field);
            lines.clear();
        }
        execute!(
            stdout(),
            SetForegroundColor(Color::White),
            SetBackgroundColor(Color::DarkBlue),
            Print(buffer.clone()),
            SetSize(SCREEN_WIDTH as u16, SCREEN_HEIGTH as u16)
        )
        .unwrap();

        // Lose state
        game_over = !can_move(piece, orientation, pos_x, pos_y, &player_field);
    }
}
