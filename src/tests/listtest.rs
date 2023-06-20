use super::super::modules::{localsearch::OrdList, localsearch::OptList};
use super::super::modules::{tsp};

use rand::Rng;
use std::collections::BTreeSet;


fn eq_tord( ord1 : &tsp::Tord, ord2 : &tsp::Tord ) -> bool {
    let mut bs : BTreeSet<(usize, usize)>  = BTreeSet::new();
    let n = ord1.len();

    if n != ord2.len() { return false; }
    for i in 0..ord1.len() {
        let next = (i+1)%n;
        let v1 = ord1[i];
        let v2 = ord1[next];

        bs.insert((v1.min(v2), v1.max(v2)));
    }


    for i in 0..ord1.len() {
        let next = (i+1)%n;
        let v1 = ord2[i];
        let v2 = ord2[next];

        if bs.contains(&(v1.min(v2), v1.max(v2))) == false {
            return false;
        };
    }
    return true
}

fn eq( v1 : &Vec<(usize,usize)>, v2 : &OrdList) -> bool {
    if v1.len() != v2.list.len() { return false; }

    for i in 0..v1.len() {
        let mut tmp1 = v1[i].clone();
        if tmp1.0 > tmp1.1 {
            tmp1 = (tmp1.1, tmp1.0);
        }
        let mut tmp2 = v2.list[i].clone();
        if tmp2.0 > tmp2.1 {
            tmp2 = (tmp2.1, tmp2.0);
        }

        if tmp1 != tmp2 {
            return false;
        }
    }

    return true
}

pub fn test_from() {
    let ord = vec![3,6,4,2,1,5,0];

    let ans = vec![(5,3), (2,5), (4,1), (0,6), (2,6), (1,0), (3,4)];
    let ordlist = OrdList::from(&ord);

    println!("OrdList From Test Function");
    println!("original ord = {:?}", ord);
    println!("    ans = {:?}", ans);
    println!("ordlist = {:?}", ordlist.list);

    let is_eq = eq(&ans, &ordlist);
    let result = if is_eq { "OK"} else {"NO"};
    println!("result : {}", result);
    println!();
    
    let ord = vec![0,6,4,2,1,5,3];
    let ans = vec![(3,6), (2,5), (4,1), (5,0), (6,2), (1,3), (0,4)];
    let ordlist = OrdList::from(&ord);
    println!("OrdList From Test Function");
    println!("original ord = {:?}", ord);
    println!("    ans = {:?}", ans);
    println!("ordlist = {:?}", ordlist.list);
    let is_eq = eq(&ans, &ordlist);
    let result = if is_eq { "OK"} else {"NO"};
    println!("result : {}", result);
}

pub fn test_convert() {

    let ord = vec![0,6,4,2,1,5,3];
    let ordlist = OrdList::from(&ord);

    let ord2 = ordlist.to_tord().unwrap();

    println!("OrdList to_Tord Test Function");
    println!("  original ord = {:?}", ord);
    println!("converted ord2 = {:?}", ord2);


    let is_eq = eq_tord(&ord, &ord2); 
    let result = if is_eq { "OK"} else {"NO"};
    println!("result : {}", result);
}

pub fn test_insert() {
    let ord = vec![0,1,2,3,4,5,6];
    let mut ordlist = OrdList::from(&ord);

    ordlist.insert(3, 0, 1 );

    let ans : tsp::Tord = vec![0, 3, 1, 2, 4, 5, 6];
    let ord2 = ordlist.to_tord().unwrap();
    println!("OrdList Insert Test Function");
    println!("     ans ord = {:?}", ans);
    println!("inserted ord = {:?}", ord2);
    let is_eq = eq_tord(&ans, &ord2); 
    let result = if is_eq { "OK"} else {"NO"};
    println!("result : {}", result);

    ordlist.insert(6, 3, 1);
    let ans : tsp::Tord = vec![0, 3, 6, 1, 2, 4, 5];
    let ord2 = ordlist.to_tord().unwrap();
    println!("OrdList Insert Test Function");
    println!("     ans ord = {:?}", ans);
    println!("inserted ord = {:?}", ord2);
    let is_eq = eq_tord(&ans, &ord2); 
    let result = if is_eq { "OK"} else {"NO"};
    println!("result : {}", result);
}

pub fn test_exchange() {

    let mut ord = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13];
    let mut ordlist = OrdList::from(&ord);

    let n = ord.len();
    let mut rng = rand::thread_rng();

    for i in 0..100 {
        let x = rng.gen_range(0, n);
        let y = rng.gen_range(0, n);

        println!("x = {}, y = {}", x, y);
        ordlist.exchange(ord[x], ord[y]);
        ord.swap(x,y);
        
        let ord2 = ordlist.to_tord().unwrap();
        println!("OrdList Exchange Test Function");
        println!("     ans ord = {:?}", ord);
        println!("inserted ord = {:?}", ord2);
        let is_eq = eq_tord(&ord, &ord2); 
        let result = if is_eq { "OK"} else {"NO"};
        println!("result : {}", result);
        println!();

        if is_eq == false {
            break;
        }
    }
}

pub fn test_optlist_from() {
    let mut ord = vec![0,1,2,3,4,5,6,7];

    let optlist = OptList::from(&ord);

    let ord2 = optlist.to_tord();


    println!("[OptList From] Test Function");
    println!("     ans ord = {:?}", ord);
    println!("inserted ord = {:?}", ord2);
    let is_eq = eq_tord(&ord, &ord2); 
    let result = if is_eq { "OK"} else {"NO"};
    println!("result : {}", result);
    println!();
}