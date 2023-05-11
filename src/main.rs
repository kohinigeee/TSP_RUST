#[allow(non_snake_case)]
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
    let problem : ProblemPath = ProblemPath::new("fnl4461".to_string());
    let fpath : String = problem.getInstPath(); 
    let inst : TspInstance = TspInstance::fromFile(&fpath)?;
    let tsp : Tsp = Tsp::from(&inst);

    println!("Problem size = {}\n", tsp.size);

    let ord_near = construct::nearest_all(&tsp);
    println!("Nearest score = {}", tsp.calcScore(&ord_near).unwrap());
    println!("\n");

    let ord = construct::Kruskal(&tsp);
    println!("Kruskal score = {}\n", tsp.calcScore(&ord).unwrap());

    let mut tmp : Insertion<i64> = Insertion::new(&tsp);
    let tmp_ord : Tord = tmp.calc_nearest(None, None, None,None, None);
    let tmp_score = tsp.calcScore(&tmp_ord).unwrap();
    println!("Insertion(nearest) score = {}\n", tmp_score);

    let tmp_ord2 : Tord = tmp.calc_nearest(None, Some(Insertion::init_points_center), None, None, None); 
    println!("Insertion(nearest init by center) score = {}\n", tsp.calcScore(&tmp_ord2).unwrap());

    let tmp_ord3 : Tord = tmp.calc_nearest(None, Some(Insertion::init_points_center_by_center), None, None, None); 
    println!("Insertion(nearest init by center with center) score = {}\n", tsp.calcScore(&tmp_ord3).unwrap());

    let ord_fartest = tmp.calc_farthest(None, None, None, None, None);
    let score_farthest = tsp.calcScore(&ord_fartest).unwrap();
    println!("Insertion(farthest) score = {}\n", score_farthest);
    let ord_fartest2 = tmp.calc_farthest(None, Some(Insertion::init_points_center), None, None, None);
    println!("Insertion(farthest init by center) score = {}\n", tsp.calcScore(&ord_fartest2).unwrap());

    let ord_dif = tmp.calc_diff(None, None, None, None, None);
    let score_dif = tsp.calcScore(&ord_dif).unwrap();
    println!("Insertion(difference) score = {}\n", score_dif);
    let ord_dif2 = tmp.calc_diff(None, Some(Insertion::init_points_center), None, None, None);
    println!("Insertion(difference init by center) score = {}\n", tsp.calcScore(&ord_dif2).unwrap());

    // let optpath = problem.getOptPath();
    // let opttour : OptTour = OptTour::fromFile(&optpath, &tsp).unwrap();
    // println!("opt score = {}", opttour.score);
    // println!("{:?}\n", opttour);

    let center : Point = tsp.center.clone();
    let p : Point = Point::new(center.x/4, center.y/4*3);
    let div_path = construct::divide4(&tsp, &p);
    let div_score = tsp.calcScore(&div_path).unwrap();
    println!("Divide to four area score = {}\n", div_score);
    // println!("divpath = {:?}", div_path);
    

    let near2_ord = construct::nearest_twoedge_all(&tsp);
    let near2_score = tsp.calcScore(&near2_ord).unwrap();
    println!("Nearest both endpoint score = {}\n", near2_score);

    Ok(())
}
