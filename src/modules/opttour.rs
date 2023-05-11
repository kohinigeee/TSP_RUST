use std::fs::File;
use std::io::{self, BufRead, BufReader};
use super::tsp::Tsp;
use super::tsp::*;

type Tord = super::tsp::Tord;

#[derive(Debug)]
pub struct OptTour {
    pub size : u32,
    pub score : i64,
    pub ord : Tord,
}

impl OptTour {
    pub fn new( size : u32, score : i64, ord : Vec<usize> ) -> OptTour {
        OptTour{ size, score , ord}
    }
    
    pub fn fromFile( fpath : &String, tsp: &Tsp ) -> Result<OptTour,Box<dyn std::error::Error>> {

        let mut ord: Vec<usize> = vec![];
        let mut size: u32 = 0;
        let mut isNoMode: bool = false;
 
        for result in BufReader::new(File::open((fpath))?).lines() {
            let l : String = result?;
            let sv : Vec<String> = l.trim().split_whitespace().map( |e| e.to_string()).collect();
            
            if isNoMode {
                let no : i32 = sv[0].parse().unwrap();
                if no == -1 { isNoMode = false; continue; }
                ord.push( (no-1) as usize );
                continue;
            } 

            match sv[0].as_str() {
                "DIMENSION" => {
                    size = sv[2].parse().unwrap();
                }
                "TOUR_SECTION" => {
                    isNoMode = true;
                },
                "EOF" => {
                    break;
                }
                _ => {}
            }
        }

        Ok(OptTour::new(size, tsp.calcScore(&ord).unwrap(), ord))
    }
}
