use std::{io::*, vec::*};

fn solve(_case_no:&i32){
    let mut line = String::new();
    stdin().read_line(&mut line);
    let meta: Vec<&str> = line.trim().split_whitespace().collect();
    let price_cj = meta[0].parse::<i32>().unwrap();
    let price_jc = meta[1].parse::<i32>().unwrap();
    let mut string = meta[2].to_string();
    string.retain(|c| c != '?');
    let mut price = 0;
    for c in string.char_indices() {
        if c.0 < string.len() - 1 && string.as_bytes()[c.0 + 1] != c.1 as u8 {
            price = price + match c.1 as char {
                'C' => price_cj,
                'J' => price_jc,
                _ => 0, 
            };
        }
    }
    println!("Case #{}: {}", _case_no, price);
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
