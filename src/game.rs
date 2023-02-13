use crate::block::{block_kind, block_kind::WALL as W, BlockColor, COLOR_TABLE};
use crate::mino::{gen_mino_7, MinoKind, MinoShape, MINOS};
use std::collections::VecDeque;

pub const NEXT_LENGTH: usize = 3;

pub const FIELD_WIDTH: usize = 12 + 2;
pub const FIELD_HEIGHT: usize = 22 + 1;

pub const SCORE_TABLE: [usize; 5] = [
    0,   // 0 line
    1,   // 1 line
    5,   // 2 lines
    25,  // 3 lines
    100, //  4lines
];

pub type FieldSize = [[BlockColor; FIELD_WIDTH]; FIELD_HEIGHT];

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn init() -> Position {
        Position { x: 5, y: 0 }
    }
}

#[derive(Clone)]

pub struct Game {
    pub field: FieldSize,
    pub pos: Position,
    pub mino: MinoShape,
    pub hold: Option<MinoShape>,
    pub holded: bool,
    pub next: VecDeque<MinoShape>,
    pub next_buf: VecDeque<MinoShape>,
    pub score: usize,
    pub total_line: usize, // total line deletion
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            field: [
                [0, W, W, W, 0, 0, 0, 0, 0, 0, W, W, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, W, 0],
                [0, W, W, W, W, W, W, W, W, W, W, W, W, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            pos: Position::init(),
            mino: MINOS[rand::random::<MinoKind>() as usize],
            hold: None,
            holded: false,
            next: gen_mino_7().into(),
            next_buf: gen_mino_7().into(),
            score: 0,
            total_line: 0,
        };
        spawn_mino(&mut game).ok();
        game
    }
}

pub fn is_collision(field: &FieldSize, pos: &Position, mino: &MinoShape) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
                continue;
            }
            if mino[y][x] != block_kind::NONE && field[y + pos.y][x + pos.x] != block_kind::NONE {
                return true;
            }
        }
    }
    false
}

#[allow(clippy::needless_range_loop)]
pub fn draw(
    Game {
        field,
        pos,
        mino,
        hold,
        holded: _,
        next,
        next_buf: _,
        score,
        total_line,
    }: &Game,
) {
    let mut field_buf = *field;

    let ghost_pos = ghost_pos(field, pos, mino);
    for y in 0..4 {
        for x in 0..4 {
            if mino[y][x] != block_kind::NONE {
                field_buf[y + ghost_pos.y][x + ghost_pos.x] = block_kind::GHOST;
            }
        }
    }

    for y in 0..4 {
        for x in 0..4 {
            if mino[y][x] != block_kind::NONE {
                field_buf[y + pos.y][x + pos.x] = mino[y][x];
            }
        }
    }

    // hold rendering
    println!("\x1b[2;28HHOLD");
    if let Some(hold) = hold {
        for y in 0..4 {
            print!("\x1b[{};28H", y + 3);
            for x in 0..4 {
                print!("{}", COLOR_TABLE[hold[y][x]]);
            }
        }
    }

    // next minos rendering
    println!("\x1b[8;28HNEXT");
    for (i, next) in next.iter().take(NEXT_LENGTH).enumerate() {
        for y in 0..4 {
            print!("\x1b[{};28H", i * 4 + y + 9);
            for x in 0..4 {
                print!("{}", COLOR_TABLE[next[y][x]]);
            }
            println!();
        }
    }

    // score rendering
    println!("\x1b[22;28H{score}");

    // totle line rendering
    println!("\x1b[24;28H{total_line} lines in total");

    // field rendering
    println!("\x1b[H");
    for y in 0..FIELD_HEIGHT - 1 {
        for x in 1..FIELD_WIDTH - 1 {
            print!("{}", COLOR_TABLE[field_buf[y][x]]);
        }
        println!();
    }

    // reset color info
    println!("\x1b[0m");
}

pub fn fix_mino(
    Game {
        field,
        pos,
        mino,
        hold: _,
        holded: _,
        next: _,
        next_buf: _,
        score: _,
        total_line: _,
    }: &mut Game,
) {
    for y in 0..4 {
        for x in 0..4 {
            if mino[y][x] != block_kind::NONE {
                field[y + pos.y][x + pos.x] = mino[y][x];
            }
        }
    }
}

pub fn erase_line(field: &mut FieldSize) -> usize {
    let mut line_count = 0;
    for y in 1..FIELD_HEIGHT - 2 {
        let mut can_erase = true;
        for x in 1..FIELD_WIDTH - 1 {
            if field[y][x] == 0 {
                can_erase = false;
                break;
            }
        }
        if can_erase {
            line_count += 1;
            for y2 in (2..=y).rev() {
                field[y2] = field[y2 - 1];
            }
        }
    }
    line_count
}

pub fn move_mino(game: &mut Game, new_pos: Position) {
    if !is_collision(&game.field, &new_pos, &game.mino) {
        game.pos = new_pos
    }
}

pub fn spawn_mino(game: &mut Game) -> Result<(), ()> {
    game.pos = Position::init();
    game.mino = game.next.pop_front().unwrap();

    if let Some(next) = game.next_buf.pop_front() {
        game.next.push_back(next);
    } else {
        game.next_buf = gen_mino_7().into();
        game.next.push_back(game.next_buf.pop_front().unwrap());
    }

    if is_collision(&game.field, &game.pos, &game.mino) {
        Err(())
    } else {
        Ok(())
    }
}

pub fn gameover(game: &Game) -> ! {
    draw(game);
    println!("Game Over!");
    quit();
}

pub fn quit() -> ! {
    println!("\x1b[?25h");
    std::process::exit(0);
}

#[allow(clippy::needless_range_loop)]
pub fn rotate_right(game: &mut Game) {
    let mut new_shape: MinoShape = Default::default();
    for y in 0..4 {
        for x in 0..4 {
            new_shape[y][x] = game.mino[4 - 1 - x][y];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.mino = new_shape;
    } else if let Ok(new_pos) = super_rotation(&game.field, &game.pos, &new_shape) {
        game.pos = new_pos;
        game.mino = new_shape;
    }
}

#[allow(clippy::needless_range_loop)]
pub fn rotate_left(game: &mut Game) {
    let mut new_shape: MinoShape = Default::default();
    for y in 0..4 {
        for x in 0..4 {
            new_shape[4 - 1 - x][y] = game.mino[y][x];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.mino = new_shape;
    } else if let Ok(new_pos) = super_rotation(&game.field, &game.pos, &new_shape) {
        game.pos = new_pos;
        game.mino = new_shape;
    }
}

pub fn hard_drop(game: &mut Game) {
    while {
        let new_pos = Position {
            x: game.pos.x,
            y: game.pos.y + 1,
        };
        !is_collision(&game.field, &new_pos, &game.mino)
    } {
        game.pos.y += 1;
    }
    let new_pos = game.pos;
    move_mino(game, new_pos);
}

pub fn landing(game: &mut Game) -> Result<(), ()> {
    fix_mino(game);
    let line_count = erase_line(&mut game.field);
    game.score += SCORE_TABLE[line_count];
    game.total_line += line_count;
    spawn_mino(game)?;
    game.holded = false;
    Ok(())
}

fn ghost_pos(field: &FieldSize, pos: &Position, mino: &MinoShape) -> Position {
    let mut ghost_pos = *pos;

    while {
        let new_pos = Position {
            x: ghost_pos.x,
            y: ghost_pos.y + 1,
        };
        !is_collision(field, &new_pos, mino)
    } {
        ghost_pos.y += 1;
    }

    ghost_pos
}

fn super_rotation(field: &FieldSize, pos: &Position, mino: &MinoShape) -> Result<Position, ()> {
    let diff_pos = [
        Position {
            x: pos.x,
            y: pos.y.checked_sub(1).unwrap_or(pos.y),
        },
        Position {
            x: pos.x + 1,
            y: pos.y,
        },
        Position {
            x: pos.x,
            y: pos.y + 1,
        },
        Position {
            x: pos.x.checked_sub(1).unwrap_or(pos.x),
            y: pos.y,
        },
    ];

    for pos in diff_pos {
        if !is_collision(field, &pos, mino) {
            return Ok(pos);
        }
    }
    Err(())
}

pub fn hold(game: &mut Game) {
    if game.holded {
        return;
    }
    if let Some(mut hold) = game.hold {
        std::mem::swap(&mut hold, &mut game.mino);
        game.hold = Some(hold);
        game.pos = Position::init();
    } else {
        game.hold = Some(game.mino);
        spawn_mino(game).ok();
    }
    game.holded = true;
}
