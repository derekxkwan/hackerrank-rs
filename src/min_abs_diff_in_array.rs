use std::io;
use std::io::prelude::*;
use std::cmp;

fn main () {
    let rdr = io::stdin();
    let mut itr = rdr.lock().lines();
    let n = itr.next().unwrap().unwrap().parse::<usize>().unwrap();
    let arraystr = itr.next().unwrap().unwrap();
    let mut array : Vec<i32> = arraystr.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    qsort(&mut array, 0, n);
    let mut cmin : i32 = 0;
    for i in 1..n {
        let cur : i32 = (array[i] - array[i-1]).abs();
        if(i == 1) {cmin = cur}
        else {cmin = cmp::min(cur, cmin)};
    };
    println!("{}", cmin);
}

fn qsort(v: &mut Vec<i32>, lo: usize, hi: usize) {
    if (hi-lo) > 1 {
        let pivot = v[lo];
        let mut i = lo + 1;
        let mut j = lo + 1;
        while j < hi {
            let cswap : bool = v[j] < pivot;
            if cswap == true {
                let temp = v[j];
                v[j] = v[i];
                v[i] = temp;
                i += 1;
            };
            j += 1;
        };
        let temp = v[i-1];
        v[i-1] = pivot;
        v[lo] = temp;
        let _a = qsort(v,lo, i-1);
        let _b = qsort(v, i, hi);
    };
}
