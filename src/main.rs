use std::io::{stdin, stdout, Write};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{ScrollUp, SetSize},
};
use rand::Rng;

fn rotate(px: usize, py: usize, r: usize) -> usize {
    match r % 4 {
        0 => py * 4 + px,
        1 => 12 + py - px * 4,
        2 => 15 - py * 4 - px,
        3 => 3 - py + px * 4,
        _ => 0,
    }
}

fn main() {
    const SCREEN_WIDTH: usize = 80;
    const SCREEN_HEIGTH: usize = 30;
    const FIELD_WIDTH: usize = 12;
    const FIELD_HEIGTH: usize = 18;
    let mut tetromino: Vec<&str> = vec![""; 7];
    tetromino[0] = "..X...X...X...X."; // Tetronimos 4x4
    tetromino[1] = "..X..XX...X.....";
    tetromino[2] = ".....XX..XX.....";
    tetromino[3] = "..X..XX..X......";
    tetromino[4] = ".X...XX...X.....";
    tetromino[5] = ".X...X...XX.....";
    tetromino[6] = "..X...X..XX.....";

    // Create Screen Buffer
    let mut screen: Vec<char> = vec![' '; SCREEN_WIDTH * SCREEN_HEIGTH];
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

    // Pick New Piece
    let pos_x = FIELD_WIDTH / 2;
    let pos_y = 0;
    let orientation = 0;
    let piece: u8 = rand::thread_rng().gen_range(0..7);

    // Draw Current Piece
    for px in 0..4 {
        for py in 0..4 {
            if tetromino[piece as usize]
                .chars()
                .nth(rotate(px, py, orientation))
                .unwrap()
                != '.'
            {
                screen[(pos_y + py + 2) * SCREEN_WIDTH + (pos_x + px + 2)] = (piece + 65) as char;
            }
        }
    }
    
    let buffer: String = screen.into_iter().collect();
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::DarkBlue),
        Print(buffer),
        SetSize(SCREEN_WIDTH as u16, SCREEN_HEIGTH as u16)
    )
    .unwrap();
    let _ = stdin().read_line(&mut "".to_string()).unwrap();
}
