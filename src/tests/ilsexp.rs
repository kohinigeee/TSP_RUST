
use rand::{SeedableRng, Rng};

use crate::modules::tsp::Tord;

use super::super::modules::{localsearch, myils, tsp::Tsp, tspinstance, construct};

use std::time::Instant;

struct ResultsInfos {
    pub avg_times: f64,
    pub avg_scores: f64,
    pub best_score: i64,
    pub avg_updated: f64,
}

impl ResultsInfos {
    pub fn new(avg_times: f64, avg_scores: f64, best_score: i64, avg_updated : f64) -> ResultsInfos {
        ResultsInfos {
            avg_times,
            avg_scores,
            best_score,
            avg_updated,
        }
    }
}

struct Results {
    pub cnt: usize,
    senryaku_type: localsearch::SenryakuType,
    func_type: usize,
    times: Vec<u128>,
    scores: Vec<i64>,
    update_cnts : Vec<usize>,
}

impl Results {
    pub fn new(senryaku_type: localsearch::SenryakuType, func_type: usize) -> Results {
        let cnt = 0;
        let times = vec![];
        let scores = vec![];
        let update_cnts  = vec![];
        Results {
            cnt,
            senryaku_type,
            func_type,
            times,
            scores,
            update_cnts,
        }
    }
}

impl Results {
    pub fn do_exp(&mut self, tsp: &Tsp, ord: &Vec<usize>, seed: u64, lists : &Vec<Vec<usize>>) {
        let limt_time = 15 * 60 * 1000;

        let mut local_inst = localsearch::OrdArrayLocal::new(tsp, ord);

        let mut local_inst = myils::Ils::new(&tsp, &ord);

        let start = Instant::now();
        let updated_cnt = local_inst.do_ils(seed, &self.senryaku_type);
        let duration = start.elapsed().as_millis();

        self.times.push(duration);
        self.scores.push(local_inst.best_score);
        self.update_cnts.push(updated_cnt);
        self.cnt += 1;
    }

    pub fn do_exp_multi(&mut self, tsp: &Tsp, ord: &Vec<usize>, seed: u64, kaisuu: u32 , lists : &Vec<Vec<usize>> ) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        for i in 0..kaisuu {
            let seed_s = rng.gen();
            println!("[Log] do_exp_multi i = {}", i+1);
            self.do_exp(&tsp, ord, seed_s, lists);
        }
        println!("[Log] finished do_exp_multi\n");
    }

    pub fn calc_infos(&self) -> ResultsInfos {
        let mut ans_times: f64 = 0.0;
        let mut ans_scores: f64 = 0.0;
        let mut best_score = 1_i64<<60;
        let mut ans_cnts : f64 = 0.0;

        let n: f64 = self.cnt as f64;

        for v in self.times.iter() {
            ans_times += (*v) as f64 / n;
        }

        for v in self.scores.iter() {
            ans_scores += (*v) as f64 / n;
            best_score = std::cmp::min(best_score, *v);
        }

        for v in self.update_cnts.iter() {
            ans_cnts += (*v) as f64 / n;
        }

        ResultsInfos::new(ans_times, ans_scores, best_score, ans_cnts)
    }
}

fn print_result( results : &Results, opt_score : i64 ) {
    let infos = results.calc_infos();

    let mut opt2_func_name : String;
    let mut senryaku_name : String;

    match  results.senryaku_type {
      localsearch::SenryakuType::Best => {
        senryaku_name = "最良移動戦略".to_string();
      }
      localsearch::SenryakuType::Fast => {
        senryaku_name = "即時移動戦略".to_string();
      }
    } 

    match results.func_type {
        1 => {
            opt2_func_name = "do_opt2_normal".to_string();
        }
        2 => {
            opt2_func_name = "do_opt2_shorter".to_string();
        }
        _ => {
            opt2_func_name = "do_opt2_shorter".to_string();
        }
    }

    println!("[実行結果]");
    println!("戦略タイプ : {} x {}", opt2_func_name, senryaku_name);
    println!("実行回数 : {}", results.cnt);
    println!("平均実行時間(ms) : {}", infos.avg_times);
    println!("平均更新回数 : {}", infos.avg_updated);
    println!("平均スコア : {}", infos.avg_scores);
    println!("ベストスコア: {}", infos.best_score);
    println!("最適値 : {}", opt_score);
    println!("ベストスコア/最適値: {}", infos.best_score as f64 / opt_score as f64);
    
}

pub fn ils_random_exp(problemName: &String, opt_score: i64) {
    let problem = tspinstance::ProblemPath::new(problemName);
    let instance = tspinstance::TspInstance::fromFile(&problem.getInstPath()).unwrap();
    

    let tsp = Tsp::from(&instance);
    // let ord = construct::nearest(&tsp, 1000, 1000, 0);
    let ord = construct::nearest(&tsp, 0);


    let neighbor_size = 50;
    let lists = localsearch::calcNeighborList(&tsp, neighbor_size);
    let start_score = tsp.calcScore(&ord).unwrap();

    let seed = 1005;
    let mut seed_gen = rand::rngs::StdRng::seed_from_u64(seed);
    let do_cnt = 3;

    let mut seed : u64 = seed_gen.gen();
    //do_2opt_normal x 即時
    println!("Do Exp in Results1");
    let mut Results1 = Results::new(localsearch::SenryakuType::Fast, 1);
    Results1.do_exp_multi(&tsp, &ord, seed, do_cnt, &lists);

    seed = seed_gen.gen();
    //do_2opt_normal x 最良
    println!("Do Exp in Results2");
    let mut Results2 = Results::new(localsearch::SenryakuType::Best, 1);
    Results2.do_exp_multi(&tsp, &ord, seed, do_cnt, &lists);
    
    println!("case : {}", problemName);
    println!("opt_score = {}", opt_score);
    println!("start_score = {}\n", start_score);
    // println!("Using NeigborLists");

    print_result(&Results1, opt_score);
    println!();

    print_result(&Results2, opt_score);
    println!();

}
