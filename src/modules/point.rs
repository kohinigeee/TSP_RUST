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
}
