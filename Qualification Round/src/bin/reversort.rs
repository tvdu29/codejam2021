use std::{slice, collections::HashSet, convert::TryInto, io::*, iter::FromIterator, usize, vec::*};

fn reversort(array: &mut [i32]) -> usize{
    let i = 0;
    let j = array.iter().position(|x| x == array.iter().min().unwrap()).unwrap();
    &array[..j as usize + 1].reverse();
    j as usize + 1
}

fn solve(_case_no:&i32){
    let mut line = String::new();
    stdin().read_line(&mut line);
    let nb_elems = line.trim().parse::<i32>().unwrap();
    line.clear();
    stdin().read_line(&mut line);
    let mut i: usize = 0;
    let mut array: Vec<i32> = line.trim().split_whitespace().into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<_>();
    let mut nb_iter = 0;
    while i < array.len() - 1 {
        let rev_ret = reversort(&mut array[i..]);
        nb_iter += rev_ret;
        i += 1;
    }
    println!("Case #{}: {}", _case_no, nb_iter);
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
