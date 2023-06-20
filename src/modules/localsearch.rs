use rand::seq::index;
use rand::{Rng, SeedableRng};
use std::cmp;
use std::sync::BarrierWaitResult;
use std::time::Instant;

use super::point::Point;
use super::tsp::{self, Tord, Tsp};

use std::{array, collections::BTreeSet};

fn change_value(p: &mut (usize, usize), target: usize, value: usize) {
    if p.0 == target {
        p.0 = value;
        return;
    }
    if p.1 == target {
        p.1 = value;
        return;
    }
}

fn find_value_no(p: &(usize, usize), target: usize) -> Option<usize> {
    if p.0 == target {
        return Some(0);
    }
    if p.1 == target {
        return Some(1);
    }
    return None;
}

fn find_value_mut(p: &mut (usize, usize), target: usize) -> Option<&mut usize> {
    if p.0 == target {
        return Some(&mut p.0);
    }
    if p.1 == target {
        return Some(&mut p.1);
    }
    return None;
}

#[derive(Debug, Clone)]
pub struct OrdList {
    pub list: Vec<(usize, usize)>,
}

impl OrdList {
    fn new(size: usize) -> OrdList {
        let list: Vec<(usize, usize)> = vec![(size, size); size];
        OrdList { list }
    }

    pub fn from(ord: &tsp::Tord) -> OrdList {
        let mut ordlist = OrdList::new(ord.len());

        if ordlist.list.len() <= 1 {
            eprint!("[Erorr] OrdList::from  ord len(={}) <= 1", ord.len());
            return ordlist;
        }

        let n = ord.len();
        let tail_no = ordlist.list.len() - 1;
        ordlist.list[ord[0]] = (ord[1], ord[tail_no]);

        for i in 1..n - 1 {
            ordlist.list[ord[i]] = (ord[i - 1], ord[i + 1]);
        }

        ordlist.list[ord[tail_no]] = (ord[0], ord[tail_no - 1]);
        return ordlist;
    }
}

//解の復元関数
//挿入近傍操作
//交換近傍操作
//2-Opt近傍操作
impl OrdList {
    pub fn to_tord(&self) -> Option<tsp::Tord> {
        let mut visited: Vec<bool> = vec![false; self.list.len()];
        let mut ans: tsp::Tord = vec![0; self.list.len()];

        let mut cnt = 0;
        let mut nowv = 0;
        while visited[nowv] == false {
            ans[cnt] = nowv;
            visited[nowv] = true;
            cnt += 1;

            let tmp = nowv;
            let next = self.list[nowv];
            nowv = if visited[next.0] { next.1 } else { next.0 };
        }
        if cnt != visited.len() {
            return None;
        }

        return Some(ans);
    }

    //頂点xをtarget1とtarget2の間に挿入
    //target1とtarget2が隣接してない場合は変更なし
    pub fn insert(&mut self, x: usize, target1: usize, target2: usize) {
        if x == target1 || x == target2 {
            eprintln!("[Error] OrdList::insert x is same value to target1 or target2");
            return;
        }
        if x >= self.list.len() {
            eprintln!("[Error] OrdList::insert  x is invalid value");
            return;
        }

        let target1_op = find_value_no(&self.list[target1], target2);
        let target2_op = find_value_no(&self.list[target2], target1);

        if target1_op.is_none() || target2_op.is_none() {
            eprintln!("[Error] OrdList::insert  target1 and target2 is not connecting");
            return;
        }

        let nextx1 = self.list[x].0;
        let nextx2 = self.list[x].1;
        change_value(&mut self.list[nextx1], x, nextx2);
        change_value(&mut self.list[nextx2], x, nextx1);

        *find_value_mut(&mut self.list[target1], target2).unwrap() = x;
        *find_value_mut(&mut self.list[target2], target1).unwrap() = x;

        self.list[x] = (target1, target2);
    }

    pub fn exchange(&mut self, x: usize, y: usize) {
        let n = self.list.len();
        if x >= n || y >= n {
            eprintln!("[Error] OrdList::exchange x or y is invalid value");
            return;
        }
        if x == y {
            return;
        }

        let nextx = self.list[x].clone();
        let nexty = self.list[y].clone();

        self.list.swap(x, y);

        change_value(&mut self.list[nextx.0], x, y);
        change_value(&mut self.list[nextx.1], x, y);

        change_value(&mut self.list[nexty.0], y, x);
        change_value(&mut self.list[nexty.1], y, x);

        change_value(&mut self.list[x], x, y);
        change_value(&mut self.list[y], y, x);
    }

    pub fn opt2(&mut self, x: usize, y: usize) {
        let n = self.list.len();
        if x >= n || y >= n {
            eprintln!("[Error] OrdList::opt2 x or y is invalid value");
            return;
        }
        if x == y {
            return;
        }
    }
}

pub struct OptList {
    //list[i][j][k] := リストi(表か裏か)の頂点iのk( 0 : next, 1 : back )
    //要素(usize, usize) = .0 -> 表か裏か, .1-> 頂点番号
    pub list: Vec<Vec<Vec<(usize, usize)>>>,
    pub length: usize,
}

impl OptList {
    pub fn from(ord: &tsp::Tord) -> OptList {
        let n = ord.len();
        println!("debug n = {}", n);
        let length = n;
        let mut list = vec![vec![vec![(0, 0); 2]; n]; 2];
        for (i, v) in ord.iter().enumerate() {
            let next_idx = (i + 1) % n;
            let back_idx = (i + n - 1) % n;
            list[0][*v][0] = (0, ord[next_idx]);
            list[0][*v][1] = (0, ord[back_idx]);

            list[1][*v][0] = (1, ord[back_idx]);
            list[1][*v][1] = (1, ord[next_idx]);
        }
        OptList { list, length }
    }
}

impl OptList {
    pub fn to_tord(&self) -> tsp::Tord {
        let mut ans: tsp::Tord = vec![];

        let mut p = (0, 0);

        let mut visited: Vec<bool> = vec![false; self.length];
        while visited[p.1] == false {
            visited[p.1] = true;
            ans.push(p.1);
            p = self.list[p.0][p.1][0];
        }
        ans
    }

    pub fn opt2(&self, x: usize, y: usize) {
        if x >= self.length || y >= self.length {
            eprintln!("[Error] OptList::opt2 x or y is invalid value");
            return;
        }
        if x == y {
            eprintln!("[Error] OptList::opt2 x and y is same value");
            return;
        }

        let next_x = self.list[0][x][0].1;
        let next_y = self.list[0][y][0].1;

        if x == next_y || y == next_x {
            return;
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SenryakuType {
    Fast,
    Best,
}

#[derive(Debug, Clone)]
pub struct OrdArrayLocal {
    pub array: Vec<usize>,
    pub pos: Vec<usize>,
    tsp: Tsp,
    now_score: i64,
    pub best_score: i64,
    pub cnt_moves: u64,
    pub cnt_rounds: u64,
}

impl OrdArrayLocal {
    pub fn new(init_tsp: &Tsp, init_ord: &Tord) -> OrdArrayLocal {
        let score = init_tsp.calcScore(init_ord).unwrap();
        let array = init_ord.clone();
        let tsp = init_tsp.clone();
        let now_score = score;
        let best_score = score;
        let cnt_moves = 0;
        let cnt_rounds = 0;

        let mut pos = vec![0; tsp.size];

        for (i, v) in array.iter().enumerate() {
            pos[*v] = i;
        }

        OrdArrayLocal {
            array,
            tsp,
            now_score,
            best_score,
            cnt_moves,
            cnt_rounds,
            pos,
        }
    }
}

impl OrdArrayLocal {
    fn get_next_index(&self, idx: usize) -> usize {
        return (idx + 1) % self.array.len();
    }

    fn get_back_index(&self, idx: usize) -> usize {
        return (self.array.len() + idx - 1) % self.array.len();
    }

    //エッジとしてはindex1の前、inddex2の次の頂点との辺が選ばれる
    pub fn calc_2opt_dif(&self, val1: usize, val2: usize) -> i64 {
        if val1 >= self.tsp.size {
            panic!("[calc_2opt_dif] 不正な値")
        }
        if val2 >= self.tsp.size {
            panic!("[calc_2opt_dif] 不正な値")
        }
        if val1 == val2 {
            return 0;
        }

        let index1 = self.pos[val1];
        let index2 = self.pos[val2];

        let index1_next = self.get_next_index(index1);
        let index2_next = self.get_next_index(index2);

        let idx1_p = &self.tsp.points[self.array[index1]];
        let idx2_p = &self.tsp.points[self.array[index2]];
        let idx1_nextp = &self.tsp.points[self.array[index1_next]];
        let idx2_nextp = &self.tsp.points[self.array[index2_next]];

        // println!("emit1 = ({}, {})", index1_val, index1_backv);
        // println!("emit2 = ({}, {})", index2_val, index2_nextv);
        let emit_dis1 = Point::dis_sqrt(idx1_p, idx1_nextp);
        let emit_dis2 = Point::dis_sqrt(idx2_p, idx2_nextp);

        // println!("add1 = ({}, {})", index1_backv, index2_val);
        // println!("add1 = ({}, {})", index1_val, index2_nextv);
        let add_dis1 = Point::dis_sqrt(idx1_nextp, idx2_nextp);
        let add_dis2 = Point::dis_sqrt(idx1_p, idx2_p);

        return add_dis1 + add_dis2 - emit_dis1 - emit_dis2;
    }

    pub fn do_2opt_normal(&mut self, val1: usize, val2: usize) -> bool {
        if val1 >= self.tsp.size {
            panic!("[calc_2opt_dif] 不正な値")
        }
        if val2 >= self.tsp.size {
            panic!("[calc_2opt_dif] 不正な値")
        }
        if val1 == val2 {
            return true;
        }

        let index1 = self.pos[val1];
        let index2 = self.pos[val2];

        let mut indexl = cmp::min(index1, index2);
        let mut indexr = cmp::max(index1, index2);
        indexl += 1;

        while indexl < indexr {
            let val_l = self.array[indexl];
            let val_r = self.array[indexr];

            self.pos.swap(val_l, val_r);
            self.array.swap(indexl, indexr);

            indexl += 1;
            indexr -= 1;
        }
        return true;
    }

    pub fn do_2opt_shorter(&mut self, val1: usize, val2: usize) -> bool {
        if val1 == val2 {
            return true;
        }

        let n = self.array.len();
        let index1 = self.pos[val1];
        let index2 = self.pos[val2];

        let mut indexl = cmp::min(index1, index2);
        let mut indexr = cmp::max(index1, index2);

        let inorder_cnt = indexr - indexl;
        let backorder_cnt = n - inorder_cnt;

        // println!(
        //     "[Log] inorder_cnt = {}, backorder_cnt = {}",
        //     inorder_cnt, backorder_cnt
        // );

        if inorder_cnt <= backorder_cnt {
            self.do_2opt_normal(val1, val2);
            return true;
        }

        let cnt = backorder_cnt / 2;
        indexr = self.get_next_index(indexr);
        for _ in 0..cnt {
            let val_l = self.array[indexl];
            let val_r = self.array[indexr];
            self.pos.swap(val_l, val_r);

            self.array.swap(indexl, indexr);
            indexl = self.get_back_index(indexl);
            indexr = self.get_next_index(indexr);
        }

        return true;
    }

    //funcno 1 : do_2opt_normal
    //       2 : do_2opt_shorter
    //senryakutype  1 : 即時
    //              2 : 最良
    pub fn opt2_random(&mut self, functype: usize, senryaku_type: &SenryakuType, seed: u64) {
        self.cnt_moves = 0;
        self.cnt_rounds = 0;
        let n = self.tsp.size;
        let limt = (n * n) as f64 * 0.1;

        let lmit_time = 5 * 60 * 1000;

        let neighbor_size = limt as u64;

        println!("Log[opt2_random] neibhor_size = {}", neighbor_size);

        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let start = Instant::now();
        loop {
            // println!("now_score = {}", self.now_score);

            let mut is_updated = false;
            let mut best_pair: (usize, usize) = (n + 1, n + 1);
            let mut best_dif = 0;

            for _ in 0..neighbor_size {
                let idx1 = rng.gen_range(0, n);
                let idx2 = rng.gen_range(0, n);

                let dif = self.calc_2opt_dif(idx1, idx2);
                self.cnt_rounds += 1;

                // println!("[Log] dif = {}", dif);

                if dif < best_dif {
                    best_pair = (idx1, idx2);
                    best_dif = dif;
                    is_updated = true;

                    if *senryaku_type == SenryakuType::Fast {
                        break;
                    }
                }

                let duration = start.elapsed().as_millis();
                if duration >= lmit_time {
                    is_updated = false;
                    break;
                }
            }

            if is_updated == false {
                break;
            }

            self.now_score += best_dif;
            self.best_score = cmp::min(self.now_score, self.best_score);
            match functype {
                1 => {
                    self.do_2opt_normal(best_pair.0, best_pair.1);
                }
                2 => {
                    self.do_2opt_shorter(best_pair.0, best_pair.1);
                }
                _ => {
                    self.do_2opt_shorter(best_pair.0, best_pair.1);
                }
            }
            self.cnt_moves += 1;
        }
    }

    pub fn opt2_nighborlists_random(
        &mut self,
        functype: usize,
        senryaku_type: &SenryakuType,
        seed: u64,
        lists: &Vec<Vec<usize>>,
    ) {
        self.cnt_moves = 0;
        self.cnt_rounds = 0;
        let n = self.tsp.size;

        let lmit_time = 5 * 60 * 1000;
        let neighbor_size = n;

        println!("Log[opt2_random] neibhor_size = {}", neighbor_size);

        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let start = Instant::now();
        'outer: loop {
            // println!("now_score = {}", self.now_score);

            let mut is_updated = false;
            let mut best_pair: (usize, usize) = (n + 1, n + 1);
            let mut best_dif = 0;

            'round: for _ in 0..neighbor_size {
                let val1 = rng.gen_range(0, n);

                //1ラウンド(探索)
                for (i, v) in lists[val1].iter().enumerate() {
                    let val2 = *v;
                    let dif = self.calc_2opt_dif(val1, val2);
                    self.cnt_rounds += 1;

                    if dif < best_dif {
                        best_pair = (val1, val2);
                        best_dif = dif;
                        is_updated = true;

                        if *senryaku_type == SenryakuType::Fast {
                            break 'round;
                        }

                        let duration = start.elapsed().as_millis();
                        if duration >= lmit_time {
                            is_updated = false;
                            break 'outer;
                        }
                    }
                }
            }

            if is_updated == false {
                break;
            }

            self.now_score += best_dif;
            self.best_score = cmp::min(self.now_score, self.best_score);
            match functype {
                1 => {
                    self.do_2opt_normal(best_pair.0, best_pair.1);
                }
                2 => {
                    self.do_2opt_shorter(best_pair.0, best_pair.1);
                }
                _ => {
                    self.do_2opt_shorter(best_pair.0, best_pair.1);
                }
            }
            self.cnt_moves += 1;
        }
    }
}

pub fn calcNeighborList(tsp: &Tsp, list_size: usize) -> Vec<Vec<usize>> {
    let n = tsp.size;
    let mut lists: Vec<Vec<usize>> = vec![vec![]; n];
    let lmt = list_size;

    for i in 0..n {
        println!("[Log] calcNeighborList i = {}", i);
        let mut v = vec![0; n];
        for j in 0..n {
            v[j] = j;
        }

        v.sort_by(|a, b| {
            let dis1 = Point::dis(&tsp.points[*a], &tsp.points[i]);
            let dis2 = Point::dis(&tsp.points[*b], &tsp.points[i]);

            dis1.cmp(&dis2)
        });

        let mut cnt = 0;

        while cnt < lmt && cnt < n {
            lists[i].push(v[cnt]);
            cnt += 1;
        }
    }

    lists
}
