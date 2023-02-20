use crate::genetic_algorigthm::{GenoSeq, GenomeKind};
use crate::{block::block_kind, game::*};

pub fn eval(game: &Game, weight: &GenoSeq) -> Game {
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

            let mut next = game.clone();
            next.mino = next.next.pop_front().unwrap();
            next.pos = Position::init();
            for rotate_count in 0..=3 {
                let mut next = next.clone();
                for _ in 0..=rotate_count {
                    rotate_right(&mut next);
                }
                for dx in -4..=5 {
                    let mut next = next.clone();
                    let new_pos = Position {
                        x: match next.pos.x as isize + dx {
                            (..=0) => 0,
                            x => x as usize,
                        },
                        y: next.pos.y + 1,
                    };
                    move_mino(&mut next, new_pos);
                    hard_drop(&mut next);
                    fix_mino(&mut next);

                    let line = erase_line_count(&next.field);
                    let height_max = field_height_max(&next.field);
                    let height_diff = diff_in_height(&next.field);
                    let dead_space = dead_space_count(&next.field);

                    // normalization
                    let mut line = normalization(line as f64, 0.0, 4.0);
                    let mut height_max = 1.0 - normalization(height_max as f64, 0.0, 20.0);
                    let mut height_diff = 1.0 - normalization(height_diff as f64, 0.0, 200.0);
                    let mut dead_space = 1.0 - normalization(dead_space as f64, 0.0, 200.0);

                    // add weights
                    line *= weight[GenomeKind::Line] as f64;
                    height_max *= weight[GenomeKind::HeightMax] as f64;
                    height_diff *= weight[GenomeKind::HeightDiff] as f64;
                    dead_space *= weight[GenomeKind::DeadSpace] as f64;

                    // calculate score
                    let score = line + height_max + height_diff + dead_space;

                    if elite.1 < score {
                        elite.0 = game.clone();
                        elite.1 = score;
                    }
                }
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

#[allow(clippy::needless_range_loop)]
pub fn diff_in_height(field: &FieldSize) -> usize {
    let mut diff = 0;
    let mut top = [0; FIELD_WIDTH - 4];
    for x in 2..FIELD_WIDTH - 2 {
        for y in 1..FIELD_HEIGHT - 2 {
            if field[y][x] != block_kind::NONE {
                top[x - 2] = FIELD_HEIGHT - y - 1;
                break;
            }
        }
    }
    for i in 0..FIELD_WIDTH - 4 - 1 {
        diff += top[i].abs_diff(top[i + 1])
    }
    diff
}

pub fn dead_space_count(field: &FieldSize) -> usize {
    let mut count = 0;
    for y in (1..FIELD_HEIGHT - 2).rev() {
        for x in 2..FIELD_WIDTH - 2 {
            if field[y][x] == block_kind::NONE {
                for y2 in (2..y).rev() {
                    if field[y2][x] != block_kind::NONE {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }
    count
}
