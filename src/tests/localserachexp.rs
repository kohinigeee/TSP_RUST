use rand::{self, Rng, SeedableRng};
use std::{time::Instant, f32::NEG_INFINITY};

use crate::modules::localsearch;

use super::{super::modules::{
    construct, localsearch::OrdArrayLocal, localsearch::SenryakuType, tsp::Tsp, tspinstance,
}, listtest};

//2optによるローカルサーチの実験関数
//即時移動・最良移動と2opt_normal・2opt_shorterによる4パターンの実験を行う
//観測データは実行時間・最良値・ラウンド数・移動回数
//各手法に対して10回行う
//初期解は疑似nearest_neighbor

struct ResultsInfos {
    pub avg_times: f64,
    pub avg_scores: f64,
    pub avg_rounds: f64,
    pub avg_moves: f64,
    pub best_score: i64,
}

impl ResultsInfos {
    pub fn new(avg_times: f64, avg_scores: f64, avg_rounds: f64, avg_moves: f64, best_score: i64) -> ResultsInfos {
        ResultsInfos {
            avg_times,
            avg_scores,
            avg_rounds,
            avg_moves,
            best_score,
        }
    }
}

struct Results {
    pub cnt: usize,
    senryaku_type: localsearch::SenryakuType,
    func_type: usize,
    times: Vec<u128>,
    scores: Vec<i64>,
    rounds: Vec<u64>,
    moves: Vec<u64>,
}

impl Results {
    pub fn new(senryaku_type: localsearch::SenryakuType, func_type: usize) -> Results {
        let cnt = 0;
        let times = vec![];
        let scores = vec![];
        let rounds = vec![];
        let moves = vec![];
        Results {
            cnt,
            senryaku_type,
            func_type,
            times,
            scores,
            rounds,
            moves,
        }
    }
}

impl Results {
    pub fn do_exp(&mut self, tsp: &Tsp, ord: &Vec<usize>, seed: u64, lists : &Vec<Vec<usize>>) {
        let mut local_inst = OrdArrayLocal::new(tsp, ord);
        let start = Instant::now();
        local_inst.opt2_nighborlists_random(self.func_type, &self.senryaku_type, seed, lists);
        let duration = start.elapsed().as_millis();

        self.times.push(duration);
        self.scores.push(local_inst.best_score);
        self.rounds.push(local_inst.cnt_rounds);
        self.moves.push(local_inst.cnt_moves);
        self.cnt += 1;
    }

    pub fn do_exp_multi(&mut self, tsp: &Tsp, ord: &Vec<usize>, seed: u64, kaisuu: u32 , lists : &Vec<Vec<usize>> ) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        for i in 0..kaisuu {
            let seed_s: u64 = rng.gen();
            println!("[Log] do_exp_multi i = {}", i+1);
            self.do_exp(&tsp, ord, seed_s, lists);
        }
        println!("[Log] finished do_exp_multi\n");
    }

    pub fn calc_infos(&self) -> ResultsInfos {
        let mut ans_times: f64 = 0.0;
        let mut ans_scores: f64 = 0.0;
        let mut ans_rounds: f64 = 0.0;
        let mut ans_moves: f64 = 0.0;
        let mut best_score = 1_i64<<60;

        let n: f64 = self.cnt as f64;

        for v in self.times.iter() {
            ans_times += (*v) as f64 / n;
        }

        for v in self.scores.iter() {
            ans_scores += (*v) as f64 / n;
            best_score = std::cmp::min(best_score, *v);
        }

        for v in self.rounds.iter() {
            ans_rounds += (*v) as f64 / n;
        }

        for v in self.moves.iter() {
            ans_moves += (*v) as f64 / n;
        }

        ResultsInfos::new(ans_times, ans_scores, ans_rounds, ans_moves, best_score)
    }
}

fn print_result( results : &Results, opt_score : i64 ) {
    let infos = results.calc_infos();

    let mut opt2_func_name : String;
    let mut senryaku_name : String;

    match  results.senryaku_type {
      SenryakuType::Best => {
        senryaku_name = "最良移動戦略".to_string();
      }
      SenryakuType::Fast => {
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
    println!("ローカルサーチ実行回数 : {}", results.cnt);
    println!("平均実行時間(ms) : {}", infos.avg_times);
    println!("平均スコア : {}", infos.avg_scores);
    println!("ベストスコア: {}", infos.best_score);
    println!("ベストスコア/最適値: {}", infos.best_score as f64 / opt_score as f64);
    println!("平均ラウンド数(探索数) : {}", infos.avg_rounds);
    println!("平均移動回数 : {}", infos.avg_moves);
    
}

pub fn local_2opt_random_exp(problemName: &String, opt_score: i64) {
    let problem = tspinstance::ProblemPath::new(problemName);
    let instance = tspinstance::TspInstance::fromFile(&problem.getInstPath()).unwrap();
    

    let tsp = Tsp::from(&instance);
    let ord = construct::psedo_nearest(&tsp, 1000, 1000, 0);

    let neighbor_size = 100;
    let lists = localsearch::calcNeighborList(&tsp, neighbor_size);
    let start_score = tsp.calcScore(&ord).unwrap();

    let seed = 5;
    let mut seed_gen = rand::rngs::StdRng::seed_from_u64(seed);

    let do_cnt = 5;

    // println!("start_score = {}\n", start_score);
    // println!("opt_score = {}\n", opt_score);

    let mut seed : u64 = seed_gen.gen();
    //do_2opt_normal x 即時
    println!("Do Exp in Results1");
    let mut Results1 = Results::new(SenryakuType::Fast, 1);
    Results1.do_exp_multi(&tsp, &ord, seed, do_cnt, &lists);

    //do_2opt_shorter x 即時
    println!("Do Exp in Results3");
    let mut Results3 = Results::new(SenryakuType::Fast, 2);
    Results3.do_exp_multi(&tsp, &ord, seed, do_cnt, &lists);


    seed = seed_gen.gen();
    //do_2opt_normal x 最良
    println!("Do Exp in Results2");
    let mut Results2 = Results::new(SenryakuType::Best, 1);
    Results2.do_exp_multi(&tsp, &ord, seed, do_cnt, &lists);


    //do_2opt_shorter x 最良
    println!("Do Exp in Results4");
    let mut Results4 = Results::new(SenryakuType::Best, 2);
    Results4.do_exp_multi(&tsp, &ord, seed, do_cnt, &lists);
    
    println!("case : {}", problemName);
    println!("opt_score = {}", opt_score);
    println!("start_score = {}\n", start_score);
    println!("Using NeigborLists");
    print_result(&Results1, opt_score);
    println!("negibor size = {}", neighbor_size);
    println!();

    print_result(&Results3, opt_score);
    println!("negibor size = {}", neighbor_size);
    println!();

    print_result(&Results2, opt_score);
    println!("negibor size = {}", neighbor_size);
    println!();

    print_result(&Results4, opt_score);
    println!("negibor size = {}", neighbor_size);
    println!();
}
