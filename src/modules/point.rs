pub type Tpoint = i64;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x : Tpoint,
    pub y : Tpoint 
}

impl Point {
    pub fn new( x : Tpoint, y : Tpoint) -> Point {
        Point{ x, y}
    }
}

impl Point {
    pub fn dis( p1 : &Point , p2 : &Point ) -> Tpoint {
        let xdis = p1.x-p2.x;
        let ydis = p1.y-p2.y;
        return xdis*xdis+ydis*ydis;
    }

    pub fn dis_sqrt( p1 : &Point , p2 : &Point ) -> Tpoint {
        ( (Point::dis(p1,p2) as f64).sqrt()+0.5) as Tpoint
    }

    pub fn calc_center( points : &Vec<Point> ) -> Point {
        let mut sumx = 0;
        let mut sumy = 0;
        let n : i64 = points.len() as i64;

        for p in points.iter() {
            sumx += p.x;
            sumy += p.y;
        }

        Point::new(sumx/n, sumy/n)
    }
}

pub fn nearest_point( points : &Vec<Point>, p : &Point ) -> usize {
    if points.is_empty() {
        return 1;
    }

    let mut idx = 0;
    let mut min_dis = Point::dis(p, &points[idx]);

    for (i, tmp_p) in points.iter().enumerate() {
        let tmp_dis = Point::dis(p, tmp_p);
        if tmp_dis < min_dis {
            idx = i;
            min_dis = tmp_dis;
        }
    } 

    return idx;
}