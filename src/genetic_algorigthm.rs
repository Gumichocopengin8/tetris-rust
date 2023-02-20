use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};
use std::ops::Index;

use crate::ai::eval;
use crate::game::*;
use getch_rs::{Getch, Key};
use std::thread;

const POPULATION: usize = 10;
const GENERATION_MAX: usize = 10;
const LINE_COUNT_MAX: usize = 256;

const CROSSOVER_RATE: usize = 70;
const CROSSOVER_LEN: usize = (POPULATION as f64 * (CROSSOVER_RATE as f64 / 100.)) as usize;

const MUTATION_RATE: usize = 10;
const MUTATION_LEN: usize = (POPULATION as f64 * (MUTATION_RATE as f64 / 100.)) as usize;

const SELECTION_RATE: usize = 20;
const SELECTION_LEN: usize = (POPULATION as f64 * (SELECTION_RATE as f64 / 100.)) as usize;

#[allow(clippy::assertions_on_constants)]
const _: () = assert!(CROSSOVER_RATE + MUTATION_RATE + SELECTION_RATE == 100);
#[allow(clippy::assertions_on_constants)]
const _: () = assert!(CROSSOVER_LEN + MUTATION_LEN + SELECTION_LEN == POPULATION);

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

#[derive(Clone)]
struct Individual {
    geno: GenoSeq,
    score: usize,
}

impl Distribution<Individual> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> Individual {
        Individual {
            geno: rand::random::<GenoSeq>(),
            score: 0,
        }
    }
}

pub fn learning() -> ! {
    let _ = thread::spawn(|| {
        let mut inds = rand::random::<[Individual; POPULATION]>();
        for gen in 1..=GENERATION_MAX {
            println!("{gen} generation:");
            for (i, ind) in inds.iter_mut().enumerate() {
                let mut game = Game::new();
                while game.total_line < LINE_COUNT_MAX {
                    let elite = eval(&game, &ind.geno);
                    game = elite;
                    if landing(&mut game).is_err() {
                        break;
                    }
                }
                ind.score = game.score;
                println!("{i}: {:?} => {}", ind.geno, game.score);
            }
            let next_genos = gen_next_generation(&inds);
            inds.iter_mut()
                .map(|i| &mut i.geno)
                .zip(next_genos)
                .for_each(|(now, next)| *now = next);
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

fn gen_next_generation(inds: &[Individual]) -> [GenoSeq; POPULATION] {
    let mut rng = rand::thread_rng();
    let mut genos = vec![];
    genos.extend_from_slice(&crossover(inds)); // 交叉
    genos.extend_from_slice(&mutation(inds)); // 突然変異
    genos.extend_from_slice(&selection(inds)); // 選択
    genos.shuffle(&mut rng);
    genos.try_into().unwrap()
}

fn crossover(inds: &[Individual]) -> [GenoSeq; CROSSOVER_LEN] {
    let mut genos = inds.iter().map(|i| i.geno).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    for i in (0..genos.len() - 1).step_by(2) {
        let mut geno1 = genos[i];
        let mut geno2 = genos[i + 1];
        let point1 = rng.gen_range(0..4);
        let point2 = rng.gen_range(point1..4);
        mem_swap_range(&mut geno1, &mut geno2, point1..=point2);
        genos[i] = geno1;
        genos[i + 1] = geno2;
    }
    genos.shuffle(&mut rng);
    genos[..CROSSOVER_LEN].try_into().unwrap()
}

fn mem_swap_range<T>(x: &mut [T], y: &mut [T], range: std::ops::RangeInclusive<usize>) {
    for i in range {
        std::mem::swap(&mut x[i], &mut y[i]);
    }
}

fn mutation(inds: &[Individual]) -> [GenoSeq; MUTATION_LEN] {
    let mut genos = inds.iter().map(|i| i.geno).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    genos.shuffle(&mut rng);
    for geno in genos.iter_mut().take(MUTATION_LEN) {
        let mut geno = *geno;
        geno[rng.gen_range(0..4)] = rand::random();
    }
    genos[..MUTATION_LEN].try_into().unwrap()
}

fn selection(inds: &[Individual]) -> [GenoSeq; SELECTION_LEN] {
    let mut new_inds = inds.to_vec();
    new_inds.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    new_inds.iter().map(|i| i.geno).collect::<Vec<_>>()[..SELECTION_LEN]
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_swap_range() {
        let tests = [
            (0..=0, [[5, 2, 3, 4], [1, 6, 7, 8]]),
            (0..=1, [[5, 6, 3, 4], [1, 2, 7, 8]]),
            (1..=1, [[1, 6, 3, 4], [5, 2, 7, 8]]),
            (1..=2, [[1, 6, 7, 4], [5, 2, 3, 8]]),
            (1..=3, [[1, 6, 7, 8], [5, 2, 3, 4]]),
            (0..=3, [[5, 6, 7, 8], [1, 2, 3, 4]]),
        ];
        for (range, [geno1_expect, geno2_expect]) in tests {
            let mut geno1 = [1, 2, 3, 4];
            let mut geno2 = [5, 6, 7, 8];
            mem_swap_range(&mut geno1, &mut geno2, range);
            assert_eq!(geno1, geno1_expect);
            assert_eq!(geno2, geno2_expect);
        }
    }
}
