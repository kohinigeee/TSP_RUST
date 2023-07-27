use std::time::Instant;

use super::{tsp::{Tord, Tsp}, localsearch::{OrdArrayLocal, SenryakuType, calcNeighborList, self}};

use rand::{Rng, SeedableRng};

pub fn next_idx( i : usize, n : usize ) -> usize {
    (i+1)%n
}

pub fn prev_idx( i : usize, n : usize ) -> usize {
    (i+n-1)%n
}

fn select_idxs_for_dbbridge ( ord : &Tord, seed : &u64 ) -> Vec<usize> {
    let mut range  = rand::rngs::StdRng::seed_from_u64(*seed);
    
    let n = ord.len();
    let mut anss : Vec<usize> = vec![ 0; 4];
    let mut idxs : Vec<usize> = vec![ 1; n];

    for  i in 0..4  {
        let a1;
        loop {
            let j : usize = range.gen_range(0, n);
            if idxs[j] == 1 {
                a1 = j;
                break;
            }
        }

        anss[i] = a1;
        idxs[prev_idx(a1, n)] = 0;
        idxs[next_idx(a1, n)] = 0;
        idxs[a1] = 0;
    }

    anss.sort();
    anss
}

pub fn do_dbbridge( ord : &Tord, seed : u64 ) -> Tord {
    let mut ans = vec![];
    let selecteds = select_idxs_for_dbbridge(ord, &seed);
    let n = ord.len();

    let a1 = selecteds[0];
    let a2 = next_idx(a1, n);

    let b1 = selecteds[1];
    let b2 = next_idx(b1, n);

    let c1 = selecteds[2];
    let c2 = next_idx(c1, n);

    let d1 = selecteds[3];
    let d2 = next_idx(d1, n);

    let mut pick_func =  | x : usize, y : usize |{
        let mut idx = x;

       while  idx != y {
        ans.push(ord[idx]);
        idx = next_idx(idx, n);
       }
       ans.push(ord[y]);
    };        

    pick_func(d2, a1);
    pick_func(c2, d1);
    pick_func(b2, c1);
    pick_func(a2, b1);

    ans
}

pub struct Ils {
    tsp : Tsp,
    now_score : i64,
    now_ord : Tord,
    pub init_score : i64,
    pub best_score : i64,
    pub best_ord : Tord,
}

impl Ils {
    pub fn new ( init_tsp: &Tsp, init_ord: &Tord ) -> Ils {
        let tsp = init_tsp.clone();
        let score = init_tsp.calcScore(&init_ord).unwrap();
        let now_ord = init_ord.clone();
        let now_score = score;
        let init_score = score;
        let best_score = score;
        let best_ord = now_ord.clone();

        Ils {
            tsp,
            now_score,
            now_ord,
            init_score,
            best_score,
            best_ord,
        }
    }
}

impl Ils {
    pub fn do_ils(&mut self, seed : u64, senryaku : &localsearch::SenryakuType) -> usize {
        let max_streak = 500;
        let limt_time = 10 * 60 * 1000; //10分

        let mut streak = 0;
        let mut updated_cnt = 0;
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let start = Instant::now();

        let neighbor_lists = calcNeighborList(&self.tsp, 50);

        while streak <= max_streak {
            let mut local = OrdArrayLocal::new( &self.tsp, &self.now_ord);

            let duration = start.elapsed().as_millis();
            if duration >= limt_time {
                return updated_cnt;
            }

            // local.opt2_random(1, &senryaku, rng.gen(), limt_time-duration);
            local.opt2_nighborlists_random(1, &senryaku, rng.gen(), &neighbor_lists, limt_time-duration);

            self.now_ord = local.array;
            self.now_score = local.best_score;

            println!("\nnow_score = {}, streak= {}", self.now_score, streak);

            if self.now_score < self.best_score {
                self.best_score = self.now_score;
                self.best_ord = self.best_ord.clone();
                streak = 0;
                updated_cnt += 1;
            }

            self.now_ord = do_dbbridge(&self.now_ord,rng.gen()); 
            streak += 1;

            let duraton = start.elapsed().as_millis();
            if duraton >= limt_time {
                return updated_cnt;
            }
        }

        updated_cnt
    }

    pub fn print(&self) {
        println!("InitScore : {}", self.init_score);
        println!("BestScore : {}", self.best_score);
    }
}