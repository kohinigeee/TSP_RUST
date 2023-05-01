use super::{tsp::*, point::Tpoint, point::Point};
use log::{info, debug, Level};

pub fn nearest( inst : &Tsp, start : usize ) -> Tord {
    let n = inst.size;
    let mut ans : Tord = vec![start];
    let mut visited : Vec<bool> = vec![false; n];

    for _ in 1..inst.size {
        let front = ans.last().unwrap();
        let mut minId : usize = n;
        let mut minv : Tpoint = -1;

        visited[*front] = true;
        for i in 0..n {
            if visited[i] { continue;}
            let dis = Point::dis(&inst.points[*front], &inst.points[i]);
            if minv < 0 {
                minId = i;
                minv = dis;
            } else if dis < minv {
                minId = i;
                minv = dis;
            }
        }
        ans.push(minId);
    }
    ans
}

pub fn nearest_all( inst : &Tsp ) -> Tord {
    let mut ans : Tord = nearest(inst, 0);
    let mut score  = inst.calcScore(&ans);

    for i in 1..inst.size {
        let ans_tmp : Tord = nearest(inst, i);
        let score_tmp = inst.calcScore(&ans_tmp);

        if score_tmp < score {ans = ans_tmp;}
    }
    ans
}

pub fn Kruskal( tsp : &Tsp ) -> Tord {
    let n = tsp.size;
    let mut edges : Vec<(usize,usize)> = vec![];

    println!("Kruskal : start Kruskal");
    for i in 0..n {
        for j in i+1..n {
            edges.push((i,j));
        }
    } 

    println!("Kruskal : start sort");
    edges.sort_by(
        |a, b| {
            let dis1 = Point::dis(&tsp.points[a.0], &tsp.points[a.1]);
            let dis2 = Point::dis(&tsp.points[b.0], &tsp.points[b.1]);

            return dis1.cmp(&dis2);
        }
    );

    let mut p : Vec<usize> = vec![0; n]; //各頂点のパスの端点
    let mut d : Vec<usize> = vec![0; n];
    let mut cntd : Vec<usize> = vec![0; 3];
    let mut roots : Vec<(usize,usize)> = vec![];

    cntd[0] = n;
    for i in 0..n { p[i] = i; }

    for (a, b) in edges.iter() {
        let u = *a; let v = *b;

        if d[u] >= 2 || d[v] >= 2  { continue; }
        if d[u] == 1 && d[v] == 1  {
            if p[u] == v { continue; }
        }

        roots.push((u,v));
        let pu = p[u]; let pv = p[v];
        p[pu] = pv; p[pv] = pu;

        cntd[d[u]] -= 1; cntd[d[v]] -= 1;
        d[u] += 1;  d[v] += 1;
        cntd[d[u]] += 1; cntd[d[v]] += 1;

        if cntd[0] == 0 && cntd[1] == 2 { break; }
    }

    tsp.make_ord_from_edges(&roots).unwrap()
}