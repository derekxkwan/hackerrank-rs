use std::io;
use std::io::prelude::*;
use std::cmp;

fn main () {
    let rdr = io::stdin();
    let mut itr = rdr.lock().lines();
    let n = itr.next().unwrap().unwrap().parse::<i32>().unwrap();
    let arraystr = itr.next().unwrap().unwrap();
    let array : Vec<i32> = arraystr.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut incl = array[0];
    let mut excl = 0;
    for (i,v) in array.iter().enumerate() {
        if(i > 0){
            let tmp = excl + v;
            excl = cmp::max(incl, excl);
            incl = tmp;
        };
    };
    let ret = cmp::max(incl, excl);
    println!("{}", ret)

}
