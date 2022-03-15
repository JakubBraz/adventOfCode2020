use std::time::{Duration, Instant};
use std::cmp::{max, min};
use std::borrow::{BorrowMut, Borrow};

pub fn main() {
    let str_input = "198753462";
    // let str_input = "389125467";
    let input: Vec<usize> = str_input.chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
    let mut vec = build_vec(&input);
    let vec: &mut Vec<usize> = vec.borrow_mut();

    steps(vec, str_input.chars().next().unwrap().to_digit(10).unwrap() as usize - 1, 100);
    let str = build_string(vec);
    let str: Vec<&str> = str.split('1').collect();
    let str: String = [str[1], str[0]].concat();
    println!("{}", str);

    let input: Vec<usize> = input.iter().map(|x| *x).chain((10..1_000_001)).collect();
    let mut vec2 = build_vec(&input);
    let vec2: &mut Vec<usize> = vec2.borrow_mut();
    let time = Instant::now();
    steps(vec2, str_input.chars().next().unwrap().to_digit(10).unwrap() as usize - 1, 10_000_000);
    println!("duration {:?}", time.elapsed());
    println!("{}", (vec2[0] + 1) * (vec2[vec2[0]] + 1));
}

fn steps(arg: &mut Vec<usize>, first_val: usize, steps: usize) {
    let mut current = first_val;
    for _ in 0..steps {
        let removed1 = arg[current];
        let removed2 = arg[removed1];
        let removed3 = arg[removed2];

        let mut temp = current;
        while !(temp as isize - 1 >= 0 && temp - 1 != removed1 && temp - 1 != removed2 && temp - 1 != removed3) {
            if temp == 0 {
                temp = arg.len();
            } else {
                temp -= 1;
            }
        }

        let destination = temp - 1;

        arg[current] = arg[removed3];
        arg[removed3] = arg[destination];
        arg[destination] = removed1;

        current = arg[current]
    }
}

fn build_vec(arg: &Vec<usize>) -> Vec<usize> {
    let vec: Vec<usize> = arg.iter().map(|x| *x - 1).collect();
    let mut result = vec![0; vec.len()];
    for i in 0..vec.len() {
        result[vec[i]] = vec[(i + 1) % vec.len()];
    }
    result
}

fn build_string(arg: &Vec<usize>) -> String {
    let mut result: String = String::new();
    let mut current: usize = 0;
    while result.len() < arg.len() {
        result.push((arg[current] + 1).to_string().chars().next().unwrap());
        current = arg[current] as usize;
    }
    result
}
