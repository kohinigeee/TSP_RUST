use super::{tsp::*, point::Tpoint, point::Point, point::nearest_point};
use log::{info, debug, Level};
use rand::Rng;

use std::{collections::{VecDeque, BinaryHeap}, cmp::Reverse};

pub fn nearest( inst : &Tsp, start : usize ) -> Tord {
    let n = inst.size;
    let mut ans : Tord = vec![start];
    let mut idxs : Vec<usize> = vec![0; n];
    
    for i in 0..n { idxs[i] = i; }
    idxs.swap(start, n-1);
    idxs.pop();

    for _ in 1..inst.size {
        let front = ans.last().unwrap();
        let mut minidx: usize = n;
        let mut minv : Tpoint = -1;

        for (i, v) in idxs.iter().enumerate() {
            let dis = Point::dis(&inst.points[*front], &inst.points[*v]);
            if minv < 0 {
                minidx = i;
                minv = dis;
            } else if dis < minv {
                minidx = i;
                minv = dis;
            }
        }
        ans.push(idxs[minidx]);
        let tmp_size = idxs.len();
        idxs.swap(minidx, tmp_size-1);
        idxs.pop();
    }
    ans
}

pub fn nearest_twoedge( tsp : &Tsp, start : usize ) -> Tord {
    let mut ans : VecDeque<usize> = VecDeque::new();
    let mut idxs : Vec<usize>  = vec![0; tsp.size];
    let n = tsp.size;

    for i in 0..tsp.size { idxs[i] = i; }

    let idxs_size = idxs.len();
    ans.push_back(start);
    idxs.swap(start, idxs_size-1);
    idxs.pop();


    for i in 1..n {
        let top_vertex = *ans.front().unwrap();
        let tail_vertex = *ans.back().unwrap();
        let mut min_idx = 0;
        let mut dir = 0;
        let mut min_dis = Point::dis(&tsp.points[idxs[min_idx]], &tsp.points[top_vertex]);

        for (j, v) in idxs.iter().enumerate() {
            let tmp_dis = Point::dis(&tsp.points[*v], &tsp.points[top_vertex]);
            if min_dis > tmp_dis {
                min_dis = tmp_dis;
                min_idx = j;
                dir = 0;
            }

            let tmp_dis = Point::dis(&tsp.points[*v], &tsp.points[tail_vertex]);
            if min_dis > tmp_dis {
                min_dis = tmp_dis;
                min_idx = j;
                dir = 1;
            }
        }

        if dir == 0 { ans.push_front(idxs[min_idx]); }
        else { ans.push_back(idxs[min_idx]); }

        let idxs_size = idxs.len();
        idxs.swap(min_idx, idxs_size-1);
        idxs.pop();
    }

    let ord : Tord = ans.into_iter().collect(); 
    ord
}


pub fn nearest_all( inst : &Tsp ) -> Tord {
    let mut ans : Tord = nearest(inst, 0);
    let mut score  = inst.calcScore(&ans);

    for i in 1..inst.size {
        let ans_tmp : Tord = nearest(inst, i);
        let score_tmp = inst.calcScore(&ans_tmp);

        if score_tmp < score {
            score = score_tmp;
            ans = ans_tmp;}
    }
    ans
}

pub fn nearest_twoedge_all( inst : &Tsp ) -> Tord {
    let mut ans : Tord = nearest_twoedge(inst, 0);
    let mut score  = inst.calcScore(&ans);

    for i in 1..inst.size {
        let ans_tmp : Tord = nearest_twoedge(inst, i);
        let score_tmp = inst.calcScore(&ans_tmp);

        if score_tmp < score {
            score = score_tmp;
            ans = ans_tmp;}
    }
    ans
}

pub fn Kruskal( tsp : &Tsp ) -> Tord {
    let n = tsp.size;
    let mut edges : Vec<(usize,usize)> = vec![];

    // println!("Kruskal : start Kruskal");
    for i in 0..n {
        for j in i+1..n {
            edges.push((i,j));
        }
    } 

    // println!("Kruskal : start sort");
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

pub fn insertion_demo (tsp : &Tsp ) -> Tord {
    let n = tsp.size;
    let inf = (1i64<<60);
    let init_points: Vec<usize> = vec![0,1,2];
    let mut next : Vec<usize> = vec![0; n]; //ordの順番の管理
    let mut score : Vec<(i64, usize)> = vec![(inf, n+1); n]; //score ().0 : value, ().1 頂点iが挿入される間の前側の頂点
    let mut selected : Vec<bool> = vec![false; n];

    for i in 0..n { next[i] = i; }

    next[init_points[0]] = init_points[1];
    next[init_points[1]] = init_points[2];
    next[init_points[2]] = init_points[0];

    for i in init_points.iter() {
        selected[*i] = true;
        for j in 0..n {
            let tmp_score = Point::dis(&tsp.points[j], &tsp.points[*i]);
            score[j] = std::cmp::min( score[j], (tmp_score, *i));
        }
    }

    let mut selected_cnt : usize = 3;
    let cmp = |a : &(i64, usize), b: &(i64, usize)| {
        if a.0 == b.0 { return a.1 < b.1; }
        a.0 < b.0
    };

    while selected_cnt < n {
        let mut idx = n+1;
        let mut best_score = (inf, n+1);

        for ( i, sco ) in score.iter().enumerate() {
            if selected[i] { continue; }
            if *sco < best_score {
                idx = i;
                best_score = *sco;
            }
        }

        let prev_idx = best_score.1;
        next[idx] = next[prev_idx];
        next[prev_idx] = idx;

        for j in 0..n {
            let tmp_score = Point::dis(&tsp.points[j], &tsp.points[idx]);
            score[j] = std::cmp::min( score[j], (tmp_score, idx));
        }

        selected_cnt += 1;
        selected[idx] = true;
    }
    
    let mut idx = next[0];
    let mut ans : Tord = vec![idx];
    
    while idx != 0 {
        ans.push(next[idx]);
        idx = next[idx];
    }
    
    ans
}

pub struct Insertion<T: Clone> {
    tsp : Tsp,
    selected_points : Vec<usize>,
    next : Vec<usize>,
    prev : Vec<usize>,
    scores : Vec<(T,usize)>,
}

impl<T : Clone> Insertion<T> {
    pub fn new(tsp_inst: &Tsp ) -> Insertion<T> {
        let mut selected_points: Vec<usize> = vec![];
        let mut next : Vec<usize> = vec![];
        let mut prev : Vec<usize> = vec![];
        let mut scores : Vec<(T,usize)> = vec![];
        let tsp : Tsp = tsp_inst.clone();
        
        Insertion { tsp , selected_points, next, prev, scores}
    }
}


type init_points<T> = fn(&mut Insertion<T>)->Vec<usize>;
type update_score_fn<T> = fn(&mut Insertion<T>, usize)->(); // usize : 新しく追加された頂点の番号
type cmp_fn<T> = fn(&T, &T)->bool;
type select_pos_fn<T> = fn(&mut Insertion<T>, usize, &(T, usize))->usize;


impl<T : Clone> Insertion<T> {
    //初期化：　距離の合計が最小の3点
    pub const init_points_nearest : init_points<T> = |sel| {
        let mut ans : Vec<usize> = vec![];
        let mut min_score = (1i64<<60);
        let n = sel.tsp.size;

        for i in 0..n {
            for j in i+1..n {
                let dis1 = Point::dis(&sel.tsp.points[i], &sel.tsp.points[j]);
                for k in j+1 .. n {
                    let dis2 = Point::dis(&sel.tsp.points[j], &sel.tsp.points[k]);
                    let dis3 = Point::dis(&sel.tsp.points[i], &sel.tsp.points[k]);

                    let sum = dis1+dis2+dis3;
                    if min_score > sum {
                        ans = vec![i,j,k];
                        min_score = sum;
                    }
                }
            }
        }
        ans
    }; 

    //初期化：平均点から近いやつ3つ
    pub const init_points_center : init_points<T> = |sel | {
        let mut sumx = 0;
        let mut sumy = 0;
        let mut ans : Vec<usize> = vec![];
        let n = sel.tsp.size;

        for p in sel.tsp.points.iter() {
            sumx += p.x;
            sumy += p.y;
        }


        let center = Point::new( sumx/(n as i64), sumy/(n as i64));
        let mut idxs = vec![0; n];

        for i in 0..n { idxs[i] = i; }

        idxs.sort_by( |a, b|{
            let disa = Point::dis(&sel.tsp.points[*a], &center);
            let disb = Point::dis(&sel.tsp.points[*b], &center);
            disa.cmp(&disb)
        });

        for i in 0..3 { ans.push(idxs[i]);}
        ans
    };
    
    pub const init_points_center_by_center : init_points<T> = |sel|{
        let n = sel.tsp.size;
        let mut ans : Vec<usize>  = vec![];
        let mut min_dis = (1i64<<60);

        for i in 0..n {
            for j in i+1..n {
                for k in j+1..n {
                    let tmpv = vec![sel.tsp.points[i].clone(), sel.tsp.points[j].clone(), sel.tsp.points[k].clone()];
                    let center_2 = Point::calc_center(&tmpv);
                    let dis = Point::dis(&center_2, &sel.tsp.center);
                    if  min_dis > dis {
                        min_dis = dis;
                        ans = vec![i,j,k];
                    }
                }
            }
        }
        ans
    }; 


}

//アップデート関数
impl Insertion<i64> {

    //最近
    pub const update_nearest : update_score_fn<i64> = | sel, idx | {
        let n = sel.tsp.size;
        for i in 0..n {
            let dis = Point::dis(&sel.tsp.points[i], &sel.tsp.points[idx]);
            if sel.scores[i].0 > dis {
             sel.scores[i] = (dis, idx);
            }
        }
    };

    //最遠
    pub const update_farthest : update_score_fn<i64> = | sel, idx | {
            let n = sel.tsp.size;
            for i in 0..n {
                let dis = Point::dis(&sel.tsp.points[i], &sel.tsp.points[idx]);
                if dis > sel.scores[i].0 {
                    sel.scores[i] = (dis, idx);
                }
            }
    };

    //最廉
    pub const update_diff : update_score_fn<i64> = | sel, idx | {
            let n = sel.tsp.size;
            let zerogen = (1i64<<60);
            sel.scores = vec![];

            for i in 0..n { sel.scores.push((zerogen, n+1)); }
            for i in 0..n {
                if sel.next[i] != i { continue; }
                for j in sel.selected_points.iter() {
                    let next_idx = sel.next[*j];
                    let dis1 = Point::dis(&sel.tsp.points[next_idx], &sel.tsp.points[*j]);
                    let dis2 = Point::dis(&sel.tsp.points[i], &sel.tsp.points[*j]); 
                    let dis3 = Point::dis(&sel.tsp.points[i], &sel.tsp.points[idx]); 
                    let diff = dis2+dis3-dis1;
                    if diff < sel.scores[i].0 {
                        sel.scores[i] = (diff, *j);
                    }
                }
            }
    };
}

//比較関数
impl Insertion<i64> {
    pub const cmp_i64_min : cmp_fn<i64> = | a, b| { *a < *b};
    pub const cmp_i64_max: cmp_fn<i64> = | a, b| { *a > *b};
}

//挿入位置決定関数
impl<T:Clone> Insertion<T> {
    pub const select_pos_by_score : select_pos_fn<T> = |sel, idx, best| {
        best.1
    };

    pub const select_pos_by_nearest : select_pos_fn<T> = |
    sel, idx, best | {
            let n = sel.tsp.size;
            let mut minidx = n+1;
            let mut mindis : i64 = (1i64<<60);
            for i in 0..n {
                if i == idx || sel.next[i] == i { continue; }
                let dis = Point::dis(&sel.tsp.points[idx], &sel.tsp.points[i]);
                if dis < mindis {
                    minidx = i;
                    mindis = dis;
                }
            }
            minidx
    };
}

// calc_score : a:=対象のidx, b:=更新時のペアのidx
impl<T: Clone> Insertion<T>{
   pub fn calc_ord( &mut self, zerogen : &T, init_points : impl Fn(&mut Insertion<T>)->Vec<usize>, update_score : impl Fn( &mut Insertion<T>, usize), cmp : impl Fn( &T, &T,)->bool, select_pos : impl Fn( &mut Insertion<T>, usize, &(T, usize))-> usize ) -> Tord
    {  
    let ans : Tord = vec![];
    let n  = self.tsp.size;

    self.selected_points = init_points(self);
    self.next = vec![0; n];
    self.prev = vec![0; n];
    self.scores = vec![(zerogen.clone(), n+1); n];

    for i in 0..n { self.next[i] = i; self.prev[i] = i; }

    for i in 0..3 {
        self.next[self.selected_points[i]] = self.selected_points[(i+1)%3];
        self.prev[self.selected_points[i]] = self.selected_points[(i+2)%3];
    }


    update_score(self, self.selected_points[0]);
    update_score(self, self.selected_points[1]);
    update_score(self, self.selected_points[2]);

    let mut selected_cnt : usize = 3;
    while selected_cnt < n {
        let mut idx = n+1;
        let mut best_score = (zerogen.clone(), n+1);

        for ( i, sco ) in self.scores.iter().enumerate() {
            if self.next[i] != i { continue; }
            if cmp( &(*sco).0, &best_score.0 ) {
                idx = i;
                best_score = (*sco).clone();
            }
        }

        let prev_dix = select_pos(self, idx, &best_score);
        let next_dix = self.next[prev_dix];
        self.next[idx] = self.next[prev_dix];
        self.next[prev_dix] = idx;

        self.prev[idx] = prev_dix;
        self.prev[next_dix] = idx;

        self.selected_points.push(idx);

        update_score(self, idx);

        selected_cnt += 1;
    }

    let mut idx : usize = self.next[0];
    let mut ans : Tord = vec![idx];

    while idx != 0 {
        ans.push(self.next[idx]);
        idx = self.next[idx];
    }

    ans
   } 
}


impl Insertion<i64> {
    // 都市選択：最近  挿入位置：最近都市の後
    pub fn calc_nearest(&mut self, zerogen_op : Option<i64>, init_op : Option<init_points<i64>>, update_op : Option<update_score_fn<i64>>, cmp_op : Option<cmp_fn<i64>>, select_op : Option<select_pos_fn<i64>> ) -> Tord {
        let zerogen = zerogen_op.unwrap_or(1i64<<60);
        let init_fn = init_op.unwrap_or(Insertion::init_points_nearest);
        let update_fn = update_op.unwrap_or(Insertion::update_nearest);
        let cmp_fn = cmp_op.unwrap_or(Insertion::cmp_i64_min);
        let select_fn = select_op.unwrap_or(Insertion::select_pos_by_score);

        self.calc_ord(&zerogen, init_fn, update_fn, cmp_fn, select_fn)
    }


    // 都市選択：最遠 挿入位置:最も近い都市の後
    pub fn calc_farthest(&mut self, zerogen_op : Option<i64>, init_op : Option<init_points<i64>>, update_op : Option<update_score_fn<i64>>, cmp_op : Option<cmp_fn<i64>>, select_op : Option<select_pos_fn<i64>> ) -> Tord {
        let zerogen = zerogen_op.unwrap_or(0);
        let init_fn = init_op.unwrap_or(Insertion::init_points_nearest);
        let update_fn = update_op.unwrap_or(Insertion::update_farthest);
        let cmp_fn = cmp_op.unwrap_or(Insertion::cmp_i64_max);
        let select_fn = select_op.unwrap_or(Insertion::select_pos_by_nearest);

        self.calc_ord(&zerogen, init_fn, update_fn, cmp_fn, select_fn)
    }

    // 都市選択: 最廉　挿入場所：最廉
    pub fn calc_diff(&mut self, zerogen_op : Option<i64>, init_op : Option<init_points<i64>>, update_op : Option<update_score_fn<i64>>, cmp_op : Option<cmp_fn<i64>>, select_op : Option<select_pos_fn<i64>> ) -> Tord {
        let zerogen = zerogen_op.unwrap_or(1i64<<60);
        let init_fn = init_op.unwrap_or(Insertion::init_points_nearest);
        let update_fn = update_op.unwrap_or(Insertion::update_diff);
        let cmp_fn = cmp_op.unwrap_or(Insertion::cmp_i64_min);
        let select_fn = select_op.unwrap_or(Insertion::select_pos_by_score);

        self.calc_ord(&zerogen, init_fn, update_fn, cmp_fn, select_fn)
    }

}

pub fn divide4( tsp : &Tsp, center : &Point ) -> Tord {
    let mut right_up : Vec<Point> = vec![];
    let mut right_up_idx : Vec<usize> = vec![];

    let mut right_down : Vec<Point> = vec![];
    let mut right_down_idx : Vec<usize> = vec![];

    let mut left_up : Vec<Point> = vec![];
    let mut left_up_idx: Vec<usize> = vec![];

    let mut left_down : Vec<Point> = vec![];
    let mut left_down_idx: Vec<usize> = vec![];

    for (i, p)  in tsp.points.iter().enumerate() {
        if center.x <= p.x {
            if center.y <= p.y {
                right_up.push(p.clone());
                right_up_idx.push(i);
            } else {
                right_down.push(p.clone());
                right_down_idx.push(i);
            }
        } else {
            if center.y <= p.y {
                left_up.push(p.clone());
                left_up_idx.push(i);
            } else {
                left_down.push(p.clone());
                left_down_idx.push(i);
            }
        }
    }

    let mut tsps : Vec<Tsp> = vec![];
    let mut origin_idxs : Vec<Vec<usize>> = vec![];
    if !right_up.is_empty() {
        tsps.push(Tsp::new(right_up.len(), right_up));
        origin_idxs.push(right_up_idx);
    } 
    if !left_up.is_empty() {
        tsps.push(Tsp::new(left_up.len(), left_up));
        origin_idxs.push(left_up_idx);
    } 
    if !left_down.is_empty() {
        tsps.push(Tsp::new(left_down.len(), left_down));
        origin_idxs.push(left_down_idx);
    } 
    if !right_down.is_empty() {
        tsps.push(Tsp::new(right_down.len(), right_down));
        origin_idxs.push(right_down_idx);
    } 

    let mut ords : Vec<Tord> = vec![];
    for t in tsps.iter() {
        ords.push(nearest_all(t));
    }
    if ords.len() == 1 {
        println!("ords.len() == 1");
        return ords[0].clone();
    }

    let mut connect_edges : Vec<(usize,usize)> = vec![];

    let mut min_score = (1i64<<60);
    let mut opt_edges: Vec<(usize, usize)> = vec![];

    //貪欲的に連結の仕方を求める
    for i in 0..ords[0].len() {
        let mut edges : Vec<(usize,usize)> = vec![];

        let mut now_idx = i; //探索中のパスの頂点のインデックス
        for j in 0..ords.len()-1 {
            let vertex_idx = ords[j][now_idx];
            let ords_idx = (j+1)%ords.len(); //探索対象のパスのインデックス
            let nearest_idx = nearest_point(&tsps[ords_idx].points, &tsps[j].points[vertex_idx]);
            edges.push((now_idx, nearest_idx)); //最近の頂点のインデックス
            let nearest_idx_in_order = find_value(&ords[ords_idx], &nearest_idx).unwrap();//最近の頂点のパス上でのインデックス


            let ord_size = ords[ords_idx].len();
            let next_front_idx = (nearest_idx_in_order + ord_size -1 )% ord_size;
            now_idx = next_front_idx;
        }

        let ord_size = ords[0].len() as i64;
        let pre_idx = ( ( i as i64 -1 + ord_size ) % ord_size ) as usize;
        edges.push((now_idx, pre_idx));

        let mut dif : i64 = 0;
        for ( i, edge) in edges.iter().enumerate() {
            let next_idx = (i+1)%tsps.len();
            let vertex_idx_to = ords[i][edge.0];
            let vertex_idx_end = ords[next_idx][edge.1];
            dif += Point::dis_sqrt(&tsps[i].points[vertex_idx_to], &tsps[next_idx].points[vertex_idx_end]);

            let ord_size = ords[i].len() as i64;
            let pre_idx_in_ord = ( (edge.0 as i64 -1 + ord_size )% ord_size ) as usize;
            let vertex_idx_end2 = ords[i][pre_idx_in_ord];
            dif -= Point::dis_sqrt(&tsps[i].points[vertex_idx_to], &tsps[i].points[vertex_idx_end2]);
        }

        if dif < min_score {
            min_score = dif;
            opt_edges = edges;
        }
    }

    //opet_edgesによって連結の仕方が求まったため、パスを復元
    for i in origin_idxs.iter() {
        let n = i.len();
        println!("origin_idx len = {}", n);
    }
    let mut ans : Tord = vec![];
    for (i, edge) in opt_edges.iter().enumerate() {
        let mut top_idx = edge.1;
        let ord_idx = (i+1)%ords.len();
        let ord_size = ords[ord_idx].len();

        for i in 0..ord_size {
            let vertex_idx = ords[ord_idx][top_idx];
            ans.push(origin_idxs[ord_idx][vertex_idx]);
            top_idx = (top_idx+1)%ord_size;
        }
    }

    return ans;
}

fn find_value<T: Eq>( values : &Vec<T>, target : &T )-> Option<usize> {
    for (i, v) in values.iter().enumerate() {
        if *v == *target {
            return Some(i);
        }
    }
    None
}

fn lower_bound<T : PartialOrd >( array : &Vec<T>, value : &T ) -> Option<usize> {
    let n = array.len();

    for i in 0..n {
        if *value <= array[i] {
            return Some(i);
        }
    }
    return None
}

pub fn psedo_nearest( tsp : &Tsp, h : usize, w : usize, start : usize) -> Tord {
    let mut min_x = tsp.points[0].x;
    let mut min_y = tsp.points[0].y;
    let mut max_x = min_x;
    let mut max_y = min_y;

    for p in tsp.points.iter() {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }

    let dx = (max_x-min_x)/(w as i64) +1;
    let dy = (max_y-min_y)/(h as i64) +1;

    let mut matrix : Vec<Vec<Vec<usize>>> = vec![vec![vec![]; w]; h];
    let mut matrix_idxs : Vec<(usize,usize)> = vec![]; // .0: r , .1: c
    for (i,p) in tsp.points.iter().enumerate() {
        let c_idx = ((p.x - min_x)/dx) as usize;
        let r_idx = ((p.y - min_y)/dy) as usize;
        matrix[r_idx][c_idx].push(i);
        matrix_idxs.push((r_idx, c_idx));
    }

    let mut ans : Tord = vec![start];
    let inf = (1i64<<60);
    let dirx = vec![0, -1, 0, 1];
    let diry = vec![-1, 0, 1, 0];

    matrix[matrix_idxs[start].0][matrix_idxs[start].1].retain( |x| *x != start);

    for i in 1..tsp.size {
        let vertex = *ans.last().unwrap();
        let pos = matrix_idxs[vertex];

        let mut min_dis = inf;
        let mut min_idx = tsp.size;
        let mut min_idx_in_matrix = tsp.size;
        // println!("origin pos = ({}, {})", pos.0, pos.1);

        if ( !matrix[pos.0][pos.1].is_empty() ) {
            for ( i, vertex_idx ) in matrix[pos.0][pos.1].iter().enumerate() {
            let tmp_dis = Point::dis(&tsp.points[*vertex_idx], &tsp.points[vertex]);
            if tmp_dis < min_dis {
                min_dis = tmp_dis;
                min_idx = *vertex_idx;
                min_idx_in_matrix = i;
            }
            }

            ans.push(min_idx);
            let tmp_matrix_idx = matrix_idxs[min_idx];
            let size = matrix[tmp_matrix_idx.0][tmp_matrix_idx.1].len();
            matrix[tmp_matrix_idx.0][tmp_matrix_idx.1].swap(min_idx_in_matrix, size-1);
            matrix[tmp_matrix_idx.0][tmp_matrix_idx.1].pop();
            continue;
        }

        for i in 1.. w.max(h)+3 {
            let mut st_pos : (i32, i32) = ((pos.0+i) as i32, (pos.1+i) as i32);
            let d : i32 = i as i32 *2;

            for j in 0..4 {
                for _ in 0..d {
                    st_pos = (st_pos.0+diry[j], st_pos.1+dirx[j]);
                    if st_pos.0 < 0 || st_pos.0 >= h as i32 || st_pos.1 < 0 || st_pos.1 >= w as i32 { continue;}
                    let now_r = st_pos.0 as usize;
                    let now_c = st_pos.1 as usize;

                    if matrix[now_r][now_c].len() == 0 { continue; }
                    for ( i, vertex_idx ) in matrix[now_r][now_c].iter().enumerate() {
                        let tmp_dis = Point::dis(&tsp.points[*vertex_idx], &tsp.points[vertex]);
                        if tmp_dis < min_dis {
                            min_dis = tmp_dis;
                            min_idx = *vertex_idx;
                            min_idx_in_matrix = i;
                        }
                    }
                }
            } 

            if min_dis != inf {
                ans.push(min_idx);
                let tmp_matrix_idx = matrix_idxs[min_idx];
                let size = matrix[tmp_matrix_idx.0][tmp_matrix_idx.1].len();
                // println!("r = {}, c = {}, vertex = {}, size = {}, min_idx_in_matrix = {}", tmp_matrix_idx.0, tmp_matrix_idx.1, min_idx, size, min_idx_in_matrix);
                matrix[tmp_matrix_idx.0][tmp_matrix_idx.1].swap(min_idx_in_matrix, size-1);
                matrix[tmp_matrix_idx.0][tmp_matrix_idx.1].pop();
                break;
            }
        }
    }

    ans
}

pub fn psedo_nearest_all ( tsp : &Tsp, h : usize, w : usize ) -> Tord {
    let mut ans : Tord = psedo_nearest(tsp, h, w, 0);
    let mut score  = tsp.calcScore(&ans);

    for i in 1..tsp.size {
        if i % 10 == 0 { println!("i = {}", i );}
        let ans_tmp : Tord = psedo_nearest(&tsp, h, w, i);
        let score_tmp = tsp.calcScore(&ans_tmp);

        if score_tmp < score {
            score = score_tmp;
            ans = ans_tmp;}
    }
    ans
}

pub fn random_psedo ( tsp : &Tsp, h : usize, w: usize, n : usize ) -> Tord {
    let mut rng = rand::thread_rng();
    let size = tsp.size;

    let mut ans : Tord = psedo_nearest(tsp, h, w, rng.gen_range(0,  size));
    let mut min_score = tsp.calcScore(&ans).unwrap();

    for i in 1..n {
        let no = rng.gen_range(0, n);
        let tmp = psedo_nearest(tsp, h, w, no);
        let tmp_score = tsp.calcScore(&tmp).unwrap();
        if min_score > tmp_score {
            min_score = tmp_score;
            ans = tmp;
        }
    }

    ans

}

//start : 初期頂点
//tansakusuu : 上から何個を次の探索範囲に利用するか
//max_width : ビームの幅
//max_depth : ビームの最大深度
//honsu : ビームの本数
pub fn chokudai_search( tsp : & Tsp, start : usize, tansakusu : usize, max_width : usize , max_depth : usize, honsu : usize ) -> Tord {
    let mut ans : Tord = vec![start];
    let mut score : i64 = 0;
    let mut visit : Vec<bool> = vec![false; tsp.size];

    visit[start] = true;
    loop {
        if  ans.len() >= tsp.size { break; }
        let mut states : Vec<Vec<Tord>> = vec![vec![]; max_depth+1]; 
        let mut depth_pq : Vec<BinaryHeap<Reverse<(i64, usize)>>> = vec![BinaryHeap::new(); max_depth+1];

        let top = vec![*ans.last().unwrap()];
        states[0].push(top);
        depth_pq[0].push(Reverse((0,0)));

        for _ in 0..honsu {
            for depth in 0..max_depth {
               if depth_pq[depth].is_empty() { continue; } 

               let state_info = depth_pq[depth].pop().unwrap().0;
               let top_ord = states[depth][state_info.1].clone();
               let top_vertex = *top_ord.last().unwrap();
               let cmp = | a : &usize, b : &usize | {
                return Point::dis(&tsp.points[*a], &tsp.points[top_vertex]) > Point::dis(&tsp.points[*b], &tsp.points[top_vertex]);
               };

               //次の頂点の中でtansakusuu個の頂点を確保
               let mut near_vertexs : BinaryHeap<(i64, usize)> = BinaryHeap::new();
               for ( i, no ) in tsp.points.iter().enumerate() {
                if visit[i] { continue; }
                let p = ( Point::dis(&tsp.points[top_vertex], no), i);
                if near_vertexs.len() < tansakusu {
                    near_vertexs.push(p);
                }


                if p.0 >= near_vertexs.peek().unwrap().0 { continue; }
                near_vertexs.push(p);
                near_vertexs.pop();
               }

               while let Some(vertex) = near_vertexs.pop() {
                let mut netxt_state = top_ord.clone();
                netxt_state.push(vertex.1);

                let tmp_state = ( state_info.0+Point::dis_sqrt(&tsp.points[vertex.1], &tsp.points[top_vertex]), states[depth+1].len() );
                states[depth+1].push(netxt_state);
                depth_pq[depth+1].push(Reverse(tmp_state));
               }
            }
        }

        let mut best_state = vec![];
        for i in (0..max_depth+1).rev() {
            if depth_pq[i].is_empty() { continue; }
           let best_info = depth_pq[i].pop().unwrap().0;
           best_state = states[i][best_info.1].clone();
           break;
        }
        
        for i in best_state.iter() { visit[*i] = true; }
        for i in 1..best_state.len() {
            ans.push(i);
        }
        ans.pop();
    } 

    ans
}