#[allow(non_snake_case)]
#[allow(unused)]
#[allow(non_camel_case_types)]
#[allow(nonstandard_style)]

use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use std::collections::{BTreeMap};
use log::{info};
use anyhow::{self, Context};
use thiserror::{Error};
use rand::{Rng};

mod modules;
mod tests;

use modules::tspinstance::{TspInstance, ProblemPath};
use modules::point::Point;
use modules::construct;
use modules::tsp::*;
use modules::opttour::OptTour;

use tests::localserachexp;

use crate::modules::construct::{Insertion, insertion_demo, nearest_all};


fn main() -> anyhow::Result<()>{

    let mut opt_scores : BTreeMap<String, i64> = BTreeMap::new();
    opt_scores.insert("a280".to_string(), 2579);
    opt_scores.insert("att48".to_string(), 10628);
    opt_scores.insert("berlin52".to_string(), 7542);
    opt_scores.insert("pr1002".to_string(),259045);
    opt_scores.insert("fnl4461".to_string(),182566);
    opt_scores.insert("brd14051".to_string(),469385);
    opt_scores.insert("pla33810".to_string(), 66048945);
    opt_scores.insert("d493".to_string(), 35002);
    opt_scores.insert("pr264".to_string(), 49135);


    let problemname = "pla33810".to_string();
    // let problemname = "fnl4461".to_string();
    // let problemname = "brd14051".to_string();
    let opt_score = *opt_scores.get(&problemname).unwrap();

    localserachexp::local_2opt_random_exp(&problemname, opt_score);
    // tests::ordarraytest::test_2opt_dif();
    // tests::ordarraytest::test_neiborlist();

    println!();
    Ok(())
}
