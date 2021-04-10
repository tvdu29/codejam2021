use std::{collections::HashSet, iter::FromIterator, io::*, vec::*};
use array2d::*;

fn solve(_case_no:&i32){
    let mut line = String::new();
    stdin().read_line(&mut line);
    let mut size: usize = line.trim().parse().unwrap();
    line.clear();
    let mut i = 0;
    while i < size {
        stdin().read_line(&mut line);
        i += 1;
    }
    let array: Vec<i32> = line.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let k: i32 = array.clone().into_iter().enumerate().map(|(i, x)| if i % (size + 1) == 0 {x} else {0}).sum();
    let grid: Array2D<i32> = Array2D::from_row_major(&array, size, size);

    let r = {
        let mut ret = 0;
        for row in grid.as_rows() {
            let set = HashSet::<i32>::from_iter(row);
            if  set.len() != size {
                ret = &ret + 1;
                println!("debug: {:?}", &ret);
            }
        }
        println!("debug: {:?}", ret);
        ret
    };
    let c = {
        let mut ret = 0;
        for col in grid.as_columns() {
            let set = HashSet::<i32>::from_iter(col);
            if  set.len() != size {
                ret = &ret + 1;
                println!("debug: {:?}", &ret);
            }
        }
        println!("debug: {:?}", ret);
        ret
    };

    
    println!("Case #{}: {} {} {}", _case_no, k, r, c);
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line);
    let nb_tests = line.trim().parse::<i32>().unwrap();
    let mut i = 0;
    while i < nb_tests {
        solve(&(&i + 1));
        i += 1;
    }
}
