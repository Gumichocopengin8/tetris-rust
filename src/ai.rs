use crate::{block::block_kind, game::*};

pub fn eval(game: &Game) -> Game {
    // (Game, line, height_max)
    let mut elite = (game.clone(), 0, FIELD_HEIGHT);

    for rotate_count in 0..=3 {
        let mut game = game.clone();
        for _ in 0..=rotate_count {
            rotate_right(&mut game);
        }
        for dx in -4..=5 {
            let mut game = game.clone();
            let new_pos = Position {
                x: match game.pos.x as isize + dx {
                    (..=0) => 0,
                    x => x as usize,
                },
                y: game.pos.y + 1,
            };
            move_mino(&mut game, new_pos);
            hard_drop(&mut game);
            fix_mino(&mut game);

            let line = erase_line_count(&game.field);
            let height_max = field_height_max(&game.field);
            if line >= elite.1 && height_max <= elite.2 {
                elite.0 = game;
                elite.1 = line;
                elite.2 = height_max;
            }
        }
    }
    elite.0
}

#[allow(clippy::needless_range_loop)]
fn erase_line_count(field: &FieldSize) -> usize {
    let mut count = 0;
    for y in 1..FIELD_HEIGHT - 2 {
        let mut can_erase = true;
        for x in 2..FIELD_WIDTH - 2 {
            if field[y][x] == block_kind::NONE {
                can_erase = false;
                break;
            }
        }
        if can_erase {
            count += 1;
        }
    }
    count
}

#[allow(clippy::needless_range_loop)]
fn field_height_max(field: &FieldSize) -> usize {
    for y in 1..FIELD_HEIGHT - 2 {
        for x in 2..FIELD_WIDTH - 2 {
            if field[y][x] != block_kind::NONE {
                return FIELD_HEIGHT - y - 1;
            }
        }
    }
    unreachable!()
}
