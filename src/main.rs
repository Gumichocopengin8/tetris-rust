use getch_rs::{Getch, Key};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::sync::{Arc, Mutex};
use std::{thread, time};

const FIELD_WIDTH: usize = 12;
const FIELD_HEIGHT: usize = 22;

type FieldSize = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

#[derive(Clone, Copy)]
enum MinoKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

impl Distribution<MinoKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MinoKind {
        match rng.gen_range(0..=6) {
            0 => MinoKind::I,
            1 => MinoKind::O,
            2 => MinoKind::S,
            3 => MinoKind::Z,
            4 => MinoKind::J,
            5 => MinoKind::L,
            _ => MinoKind::T,
        }
    }
}

const MINOS: [[[usize; 4]; 4]; 7] = [
    [
        // I mino
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [1, 1, 1, 1],
        [0, 0, 0, 0],
    ],
    [
        // O mino
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    [
        // S mino
        [0, 0, 0, 0],
        [0, 1, 1, 0],
        [1, 1, 0, 0],
        [0, 0, 0, 0],
    ],
    [
        // Z mino
        [0, 0, 0, 0],
        [1, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    [
        // J mino
        [0, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    [
        // L mino
        [0, 0, 0, 0],
        [0, 0, 1, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],
    [
        // T mino
        [0, 0, 0, 0],
        [0, 1, 0, 0],
        [1, 1, 1, 0],
        [0, 0, 0, 0],
    ],
];

struct Position {
    x: usize,
    y: usize,
}

fn is_collision(field: &FieldSize, pos: &Position, mino: MinoKind) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
                continue;
            }
            if field[y + pos.y][x + pos.x] & MINOS[mino as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}

fn draw(field: &FieldSize, pos: &Position, mino: MinoKind) {
    let mut field_buf = field.clone();
    for y in 0..4 {
        for x in 0..4 {
            field_buf[y + pos.y][x + pos.x] |= MINOS[mino as usize][y][x];
        }
    }

    println!("\x1b[H");
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            if field_buf[y][x] == 1 {
                print!("[]");
            } else {
                print!(" .");
            }
        }
        println!();
    }
}

fn main() {
    let field = Arc::new(Mutex::new([
        [1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ]));

    let pos = Arc::new(Mutex::new(Position { x: 4, y: 0 }));
    let mino = Arc::new(Mutex::new(rand::random::<MinoKind>()));

    // clear console screen
    println!("\x1b[2J\x1b[H\x1b[?25l");
    draw(
        &field.lock().unwrap(),
        &pos.lock().unwrap(),
        *mino.lock().unwrap(),
    );
    {
        let field = Arc::clone(&field);
        let pos = Arc::clone(&pos);
        let mino = Arc::clone(&mino);

        let _ = thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(1000));
            let mut field = field.lock().unwrap();
            let mut mino = mino.lock().unwrap();
            let mut pos = pos.lock().unwrap();
            let new_pos = Position {
                x: pos.x,
                y: pos.y + 1,
            };
            if !is_collision(&field, &new_pos, *mino) {
                *pos = new_pos;
            } else {
                for y in 0..4 {
                    for x in 0..4 {
                        field[y + pos.y][x + pos.x] |= MINOS[*mino as usize][y][x];
                    }
                }
                for y in 1..FIELD_HEIGHT - 1 {
                    let mut can_erase = true;
                    for x in 0..FIELD_WIDTH {
                        if field[y][x] == 0 {
                            can_erase = false;
                            break;
                        }
                    }
                    if can_erase {
                        for y2 in (2..=y).rev() {
                            field[y2] = field[y2 - 1]
                        }
                    }
                }
                *pos = Position { x: 4, y: 0 };
                *mino = rand::random();
            }
            draw(&field, &pos, *mino);
        });
    }

    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Left) => {
                let field = field.lock().unwrap();
                let mino = mino.lock().unwrap();
                let mut pos = pos.lock().unwrap();
                let new_pos = Position {
                    x: pos.x.checked_sub(1).unwrap_or(pos.x),
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *mino) {
                    *pos = new_pos
                }
                draw(&field, &pos, *mino);
            }
            Ok(Key::Down) => {
                let field = field.lock().unwrap();
                let mino = mino.lock().unwrap();
                let mut pos = pos.lock().unwrap();
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, *mino) {
                    *pos = new_pos
                }
                draw(&field, &pos, *mino);
            }
            Ok(Key::Right) => {
                let field = field.lock().unwrap();
                let mino = mino.lock().unwrap();
                let mut pos = pos.lock().unwrap();
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, *mino) {
                    *pos = new_pos
                }
                draw(&field, &pos, *mino);
            }
            Ok(Key::Char('q')) => {
                println!("\x1b[?25h");
                return;
            }
            _ => (),
        }
    }
}
