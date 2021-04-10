use std::{io::*};

fn solve(_case_no:&i32){

    println!("Case #{}:", _case_no);
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
