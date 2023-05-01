use std::fs::File;
use std::io::{self, BufRead, BufReader};
use log::{info};

use super::point::{Point, self};

type Tpoint = super::point::Tpoint;

#[derive(Debug)]
pub struct ProblemPath {
    name : String, 
}

impl ProblemPath
{
    pub fn new( name : String ) -> ProblemPath {
        ProblemPath { name } 
    }    

    pub fn getInstPath(&self) -> String {
        let pre = "data/tsp/".to_string();
        let tmp : String = self.name.clone();
        let tail : String = ".tsp".to_string();
        let ans = pre + &tmp + &tail;
        ans
    }

    pub fn getOptPath(&self) -> String {
        let pre = "data/opt/".to_string();
        let tmp : String = self.name.clone();
        let tail : String = ".opt.tour".to_string();
        let ans = pre + &tmp + &tail;
        ans
    }
}

#[derive(Debug)]
pub struct TspInstance {
    pub name : String,
    pub comment : String,
    pub ptype : String,
    pub dimension : i32,
    pub edge_weight_type : String,
    pub points : Vec<Point>,
}

impl TspInstance {
    fn new( name : String, comment : String, ptype : String, dimension : i32, edge_weight_type : String, points : Vec<Point> ) -> TspInstance {
        TspInstance { name, comment, ptype, dimension, edge_weight_type, points}
    }
}

impl TspInstance {
    pub fn fromFile( fpath : &String) -> Result<TspInstance,  Box<dyn std::error::Error>> {

        let mut name = String::new(); 
        let mut comment = String::new(); 
        let mut ptype = String::new();  
        let mut edge_weight_type = String::new();
        let mut dimension : i32 = 0;
        let mut points : Vec<Point> = vec![];

    for result in BufReader::new(File::open(fpath)?).lines() {
        let l : String = result?;
        let sv : Vec<String> = l.trim().split_whitespace().map(|e| e.to_string()).collect();

        let pat : &String = &sv[0];
        let res_parse : Result<i32, _> = pat.parse();

        if let Err(_) = res_parse {
            continue;
        }

        if let Ok( value ) = res_parse {
            let x : Tpoint = sv[1].parse().unwrap();
            let y : Tpoint = sv[2].parse().unwrap();
            let mut p : Point = Point::new(x,y);
            points.push(p);
            continue;
        } 
        
        match pat.as_str() {
            "NAME" => {
                name = sv[2].clone();
            },
            "COMMENT" => {
                // comment = sv[2].clone();
            },
            "TYPE" => {
                ptype = sv[2].clone();
            },
            "DIMENSION" => {
                dimension = sv[2].parse().unwrap();
            },
            "EDGE_WEIGHT_TYPE" => {
                edge_weight_type = sv[2].clone();
            },
            "EOF" => {
                break;
            }
            _ => {}
        }
    }

    println!("INFO: Have read {}", fpath);
    Ok(TspInstance::new(name, comment, ptype, dimension, edge_weight_type, points))
}

}

impl TspInstance {
    pub fn clonePoints(&self) -> Vec<Point> {
        return self.points.clone(); 
    }
}

