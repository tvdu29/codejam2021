use std::{io::*};

fn solve(_case_no:&i32){
    let mut line = String::new();
    stdin().read_line(&mut line);
    let len_array = line.trim().parse::<usize>().unwrap();
    line.clear();
    stdin().read_line(&mut line);
    let mut nums_raw: Vec<&str> = line.trim().split(" ").collect::<Vec<_>>();
    let mut nums = nums_raw.iter().map(|e| e.to_string()).collect::<Vec<_>>();
    let mut ret = 0usize;
    for i in 1..len_array {
        let init_size = nums[i].len();
        while nums[i].parse::<usize>().unwrap() <= nums[i - 1].parse::<usize>().unwrap() {
            if nums[i].len() < nums[i - 1].len(){
                let c = nums[i - 1].char_indices().filter_map(|e| if e.0 == nums[i].len() {Some(e.1)} else {None}).collect::<Vec<char>>()[0];
                nums[i].push_str(c.to_string().as_str());
                ret += 1;
            } else if nums[i].len() == init_size || nums[i].len() < nums[i - 1].len() || (nums[i].len() == nums[i - 1].len() && nums[i].chars().last().unwrap() == '9') {
                //println!("add 1 {:?}", nums[i]);
                nums[i].push_str("0");
                ret += 1;
            } else  {
                //println!("plus 1 {:?}", nums[i]);
                let c = nums[i].pop().unwrap().to_digit(10).unwrap() + 1;
                nums[i].push_str(&c.to_string());
            }
        }
    };
    println!("debug: {:?}", nums);
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
