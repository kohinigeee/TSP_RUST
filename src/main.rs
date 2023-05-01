use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use log::{info};

mod modules;

use modules::tspinstance::{TspInstance, ProblemPath};
use modules::point::Point;
use modules::construct;
use modules::tsp::*;
use modules::opttour::OptTour;
use modules::segmenttree::SegmentTree;

fn main() {

    // let problem : ProblemPath = ProblemPath::new("berlin52".to_string());
    let problem : ProblemPath = ProblemPath::new("fnl4461".to_string());
    let fpath : String = problem.getInstPath(); 
    let inst : TspInstance = TspInstance::fromFile(&fpath).unwrap();
    let tsp : Tsp = Tsp::from(&inst);

    let ord = construct::Kruskal(&tsp);

    println!("size = {}", tsp.size);
    for i in ord.iter() {
        print!("{} ", i)
    }
    println!();
    println!("Kruskal score = {}", tsp.calcScore(&ord).unwrap());

    // let ord = construct::nearest_all(&tsp);
    // let score = tsp.calcScore(&ord).unwrap();

    // println!("nearest score = {}", score);
    // println!("{:?}", ord);

    // let optpath = problem.getOptPath();
    // let opttour : OptTour = OptTour::fromFile(&optpath, &tsp).unwrap();

    // println!("opt score = {}", opttour.score);
    // println!("{:?}", opttour);

}
