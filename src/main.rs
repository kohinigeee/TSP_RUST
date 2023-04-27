use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod modules;

use modules::tspinstance::TspInstance;
use modules::point::Point;

fn main() {
    println!("test");
    
    let fpath : String = String::from("./data/a280.tsp");

    let inst : TspInstance = TspInstance::fromFile(fpath).unwrap();

    let points : Vec<Point> = inst.clonePoints();
    for v in points.iter() {
        println!("({}, {})", v.x, v.y );
    } 
}
