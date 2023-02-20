use std::ops::Index;

use crate::ai::eval;
use crate::game::*;
use getch_rs::{Getch, Key};
use std::thread;

const POPULATION: usize = 10;
const LINE_COUNT_MAX: usize = 256;

pub enum GenomeKind {
    Line,
    HeightMax,
    HeightDiff,
    DeadSpace,
}

pub type GenoSeq = [u8; 4];

impl Index<GenomeKind> for GenoSeq {
    type Output = u8;
    fn index(&self, kind: GenomeKind) -> &Self::Output {
        &self[kind as usize]
    }
}

pub fn learning() -> ! {
    let _ = thread::spawn(|| {
        let genos = rand::random::<[GenoSeq; POPULATION]>();
        for (i, geno) in genos.iter().enumerate() {
            let mut game = Game::new();
            while game.total_line < LINE_COUNT_MAX {
                let elite = eval(&game, geno);
                game = elite;
                if landing(&mut game).is_err() {
                    break;
                }
            }
            println!("{i}: {:?} => {}", geno, game.score);
        }
        quit();
    });

    let g = Getch::new();
    loop {
        if let Ok(Key::Char('q')) = g.getch() {
            quit();
        }
    }
}
