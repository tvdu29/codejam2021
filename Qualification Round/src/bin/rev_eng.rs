use std::{io::*, vec::*};

fn get_max(i: i32) ->i32{
    match i > 2 {
        true => i + get_max(i - 1),
        false => 2, 
    }
}

fn reverse(size: i32, mut cpt: i32) -> String {
    let mut arr: Vec<i32> = (1..size + 1).collect();
    let mut i = 1;
    while cpt > 0 {
        let pad = (i - 1) / 2;
        if i % 2 == 0 {arr.reverse()};
        let min = if cpt < size - i as i32 {cpt} else {size - i};
        arr[(size - min - pad - 1) as usize..(size - pad) as usize].reverse();
        cpt = cpt - min;
        if i % 2 == 0 {arr.reverse()};
        i += 1;
    }
    let mut ret = String::new();
    for t in arr.iter().enumerate() {
        ret += &t.1.to_string();
        if t.0 < (size - 1) as usize {
            ret += " ";
        }
    }
    ret
}

fn solve(_case_no:&i32){
    let mut line = String::new();
    stdin().read_line(&mut line);
    let meta: Vec<i32> = line.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<_>();
    let size = meta[0];
    let score = meta[1];
    let min = size - 1;
    let max = get_max(size);
    let ret = match score >= min && score <= max {
        true => reverse(size, score - min),
        false => "IMPOSSIBLE".to_string(),
    };
    println!("Case #{}: {}", _case_no, ret);
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
