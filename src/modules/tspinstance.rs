use std::fs::File;
use std::io::{self, BufRead, BufReader};

use super::point::{Point, self};

type Tpoint = super::point::Tpoint;

#[derive(Debug)]
pub struct TspInstance {
    name : String,
    comment : String,
    ptype : String,
    dimension : i32,
    edge_weight_type : String,
    points : Vec<Point>,
}

impl TspInstance {
    fn new( name : String, comment : String, ptype : String, dimension : i32, edge_weight_type : String, points : Vec<Point> ) -> TspInstance {
        TspInstance { name, comment, ptype, dimension, edge_weight_type, points}
    }
}

impl TspInstance {
    pub fn fromFile( fpath : String) -> Result<TspInstance,  Box<dyn std::error::Error>> {

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
            let y : Tpoint = sv[1].parse().unwrap();
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
            _ => {}
        }
    }

    Ok(TspInstance::new(name, comment, ptype, dimension, edge_weight_type, points))
}

}

impl TspInstance {
    pub fn clonePoints(self) -> Vec<Point> {
        return self.points.clone(); 
    }
}

