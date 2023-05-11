use super::{point::Point, tspinstance::TspInstance};
use std::collections::VecDeque;

type Tpoint = super::point::Tpoint;
pub type Tord = Vec<usize>;

#[derive(Debug,Clone)]
pub struct Tsp {
    pub size : usize,
    pub points : Vec<Point>,
    pub center : Point,
}

//静的メンバ関数
impl Tsp {
    pub fn new( size : usize, points : Vec<Point> ) -> Tsp {
        let center = Point::calc_center(&points);
        Tsp { size, points, center }
    }

    pub fn from( inst : &TspInstance ) -> Tsp {
        let points = inst.clonePoints();
        Tsp::new(points.len(), points)
    } 
}

//メンバ関数
impl Tsp { 
    pub fn isCorrect( &self, ord : &Tord ) -> bool {
        if  ord.len() != self.size { return false; }
        let mut cnts : Vec<usize> = vec![0; self.size];

        for  v in ord.iter() {
            if *v >= self.size { return false; }
            if cnts[*v] != 0 { return false; }
            cnts[*v] += 1;
        }

        true
    }

    pub fn calcScore( &self, ord : &Tord ) -> Option<i64> {
        if !self.isCorrect(ord) { return None; }
        let mut ans : i64 = 0;

        for (i, v) in ord.iter().enumerate() {
            let nextp = (i+1)%self.size;
            ans += Point::dis_sqrt(&self.points[*v], &self.points[ord[nextp]]);
        }
        Some(ans)
    } 

    //edges : 閉路から、一辺ないエッジの集合
    pub fn make_ord_from_edges( &self, edges : &Vec<(usize,usize)>) -> Option<Tord> {
        let n : usize = self.size;
        let mut to : Vec<Vec<usize>> = vec![vec![]; n];
        let mut d : Vec<usize> = vec![0; n];

        for ( a, b ) in edges.iter() {
            to[*a].push(*b);
            to[*b].push(*a);
            d[*a] += 1;
            d[*b] += 1;
        }

        let mut st = 0;
        for (i, v) in d.iter().enumerate() {
            if *v != 2 { st = i; }
        }

        let mut ord : Tord = vec![];
        let mut node : usize = st;
        let mut visited : Vec<bool> = vec![false; n];
        let mut queue : VecDeque<usize> = VecDeque::new();

        queue.push_back(st);

        while let Some(u) = queue.pop_front() {
            visited[u] = true;
            ord.push(u);

            for v in to[u].iter() {
                if visited[*v] { continue; }
                queue.push_back(*v);
            }
        }

        if self.isCorrect(&ord) {
            return Some(ord);
        }
        
        None
    }
}