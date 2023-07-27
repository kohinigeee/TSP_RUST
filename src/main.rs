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

use modules::tspinstance::*;
use modules::point::Point;
use modules::construct;
use modules::tsp::*;
use modules::myils;

use crate::modules::construct::{Insertion, insertion_demo, nearest_all};
use crate::modules::tspinstance;

use tests::listest;
use tests::ilsexp;


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


    let problemname = "pr1002".to_string();
    // let problemname = "pla33810".to_string();
    // let problemname = "fnl4461".to_string();
    // let problemname = "brd14051".to_string();
    let opt_score = *opt_scores.get(&problemname).unwrap();

    // myils::select_idxs_for_dbbridge(&init_ord, &1u64);
    // println!();

    ilsexp::ils_random_exp(&problemname, opt_score);

    Ok(())
}
