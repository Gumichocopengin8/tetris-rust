use crate::{block::block_kind, game::*};

pub fn eval(game: &Game) -> Game {
    // (Game, score)
    let mut elite = (game.clone(), 0f64);

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

            // normaliztion
            let mut line = normalization(line as f64, 0.0, 4.0);
            let mut height_max = 1.0 - normalization(height_max as f64, 0.0, 20.0);

            // add weight
            line *= 100.0;
            height_max *= 1.0;

            // calculate score
            let score = line + height_max;

            if elite.1 < score {
                elite.0 = game;
                elite.1 = score;
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

fn normalization(value: f64, min: f64, max: f64) -> f64 {
    (value - min) / (max - min)
}
