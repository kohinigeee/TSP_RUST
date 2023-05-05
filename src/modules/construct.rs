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

    //初期化：重心?から近いやつ3つ
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