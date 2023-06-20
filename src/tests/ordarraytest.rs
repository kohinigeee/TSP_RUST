use crate::modules::construct::nearest;
use crate::modules::localsearch;

use super::super::modules::{localsearch::OrdArrayLocal};
use super::super::modules::{tspinstance, tsp, point::Point};

use rand::{self, SeedableRng, Rng};

pub fn test_2opt_dif() {
    let problem = tspinstance::ProblemPath::new(&"berlin52".to_string());
    let instance = tspinstance::TspInstance::fromFile(&problem.getInstPath()).unwrap();

    let tsp = tsp::Tsp::from(&instance);
    let ord = nearest(&tsp, 0);
    let start_score = tsp.calcScore(&ord).unwrap();
    
    let mut local_inst = OrdArrayLocal::new(&tsp, &ord);
    
    let mut now_score = start_score;

    println!("start_ord = {:?}\n", ord);
    let mut rng = rand::rngs::StdRng::seed_from_u64(10);
    let n = tsp.size;

    for i in 0..100 {
        let idx1 = rng.gen_range(0, n);
        let idx2 = rng.gen_range(0, n);

        let dif = local_inst.calc_2opt_dif(idx1, idx2);
        local_inst.do_2opt_shorter(idx1, idx2);
        now_score += dif;
        let calc_score = tsp.calcScore(&local_inst.array).unwrap();

        println!("ord = {:?}", local_inst.array);
        println!("i = {}, idxs =({},{}), now_score = {}, calc_score = {}\n", i, idx1, idx2, now_score, calc_score);
        if now_score != calc_score {

            println!("las ord = {:?}", local_inst.array);
            break;
        }
    }

    // let dif = local_inst.calc_2opt_dif(0, 3);
    // local_inst.do_2opt_normal(0, 3);
    // now_score += dif;
    
    // let score = tsp.calcScore(&local_inst.array).unwrap();
    // println!("now_ord = {:?}", local_inst.array);
    // println!("startscore = {}", start_score);
    // println!("score = {}, nowscore = {}", score, now_score);
}



pub fn test_2opt() {
    let problem = tspinstance::ProblemPath::new(&"berlin52".to_string());
    let instance = tspinstance::TspInstance::fromFile(&problem.getInstPath()).unwrap();

    let tsp = tsp::Tsp::from(&instance);
    let ord = nearest(&tsp, 0);

    let mut local_inst = OrdArrayLocal::new(&tsp, &ord);

    let mut l = 3;
    let mut r = 10;
    println!("array = {:?}", local_inst.array);
    println!("do_2opt({}, {})", l, r);
    local_inst.do_2opt_shorter(l, r);


    println!();
    l = 4; r = 48;
    println!("array = {:?}", local_inst.array);
    println!("do_2opt({}, {})", l, r);
    local_inst.do_2opt_shorter(l, r);
    println!("array = {:?}", local_inst.array);

    println!();
    l = 0; r = local_inst.array.len()-1;
    println!("array = {:?}", local_inst.array);
    println!("do_2opt({}, {})", l, r);
    local_inst.do_2opt_shorter(l, r);
    println!("array = {:?}", local_inst.array);
}

pub fn test_neiborlist() {
    let problem = tspinstance::ProblemPath::new(&"berlin52".to_string());
    let instance = tspinstance::TspInstance::fromFile(&problem.getInstPath()).unwrap();

    let tsp = tsp::Tsp::from(&instance);
    let ord = nearest(&tsp, 0);
    let start_score = tsp.calcScore(&ord).unwrap();

    let lists = localsearch::calcNeighborList(&tsp, 20);

    for (i, v) in lists[0].iter().enumerate() {
        let dis = Point::dis_sqrt(&tsp.points[*v], &tsp.points[0]);
        println!("v = {}, dis = {}", *v, dis);
    }
}