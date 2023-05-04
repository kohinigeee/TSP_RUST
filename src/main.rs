use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use log::{info};
use anyhow::{self, Context};
use thiserror::{Error};

mod modules;

use modules::tspinstance::{TspInstance, ProblemPath};
use modules::point::Point;
use modules::construct;
use modules::tsp::*;
use modules::opttour::OptTour;
use modules::segmenttree::SegmentTree;

use crate::modules::construct::{Insertion, insertion_demo};

fn main() -> anyhow::Result<()>{

    // let problem : ProblemPath = ProblemPath::new("berlin52".to_string());
    let problem : ProblemPath = ProblemPath::new("att48".to_string());
    let fpath : String = problem.getInstPath(); 
    let inst : TspInstance = TspInstance::fromFile(&fpath)?;
    let tsp : Tsp = Tsp::from(&inst);

    let ord_near = construct::nearest_all(&tsp);
    println!("Nearest score = {}", tsp.calcScore(&ord_near).unwrap());
    for i in ord_near.iter() {
        print!("{} ", i)
    }
    println!("\n");

    let ord = construct::Kruskal(&tsp);
    println!("Kruskal score = {}", tsp.calcScore(&ord).unwrap());
    for i in ord.iter() {
        print!("{} ", i)
    }
    println!("\n");

    let mut tmp : Insertion<i64> = Insertion::new(&tsp);
    let tmp_ord : Tord = tmp.calc_nearest();

    let tmp_score = tsp.calcScore(&tmp_ord).unwrap();
    println!("Insertion(nearest) score = {}", tmp_score);
    for i in tmp_ord.iter() {
        print!("{} ", i);
    }
    println!("\n");

    let ord_fartest = tmp.calc_farthest();
    let score_farthest = tsp.calcScore(&ord_fartest).unwrap();
    println!("Insertion(farthest) score = {}", score_farthest);
    for i in ord_fartest.iter() {
        print!("{} ", i);
    }
    println!("\n");

    let ord_dif = tmp.calc_diff();
    let score_dif = tsp.calcScore(&ord_dif).unwrap();
    println!("Insertion(difference) score = {}", score_dif);
    for i in ord_dif.iter() {
        print!("{} ", i);
    }
    println!("\n");

    // let ord = construct::nearest_all(&tsp);
    // let score = tsp.calcScore(&ord).unwrap();

    // println!("nearest score = {}", score);
    // println!("{:?}", ord);

    let optpath = problem.getOptPath();
    let opttour : OptTour = OptTour::fromFile(&optpath, &tsp).unwrap();

    println!("opt score = {}", opttour.score);
    println!("{:?}", opttour);


    Ok(())
}
