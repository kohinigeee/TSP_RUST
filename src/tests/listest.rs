use rand::{self, Rng, SeedableRng};
use std::{time::Instant, f32::NEG_INFINITY};

use super::super::{
    construct,tspinstance,Tsp,myils,
}; 

pub fn testDoudlbeBridge( problem_name : &String ) {
    let problem = tspinstance::ProblemPath::new(problem_name);
    let instance = tspinstance::TspInstance::fromFile(&problem.getInstPath()).unwrap();
    
    let tsp = Tsp::from(&instance);

    let mut ord = construct::nearest(&tsp, 0);

    let seed = 120;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    for i in 0..100 {
        println!("[testDoubleBridge] i = {}", i+1);
        let seedv : u64 = rng.gen();
        ord = myils::do_dbbridge(&ord, seedv); 

        if tsp.isCorrect(&ord) {
            println!("Result : Ok");
        } else {
            println!("Result : No");
            println!("Test Failed");
            return;
        }
    }

    println!("Test Succeeded");
}