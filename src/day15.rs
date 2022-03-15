use std::collections::HashMap;

pub fn part1() {
    let inp = input();
    let m = &mut read_init(&inp[0..inp.len()]);
    println!("{}", get_nth(*inp.last().unwrap(), inp.len() as u32 + 1, 2020, m));
}

pub fn part2() {
    let inp = input();
    let m = &mut read_init(&inp[0..inp.len()]);
    println!("{}", get_nth2(*inp.last().unwrap(), inp.len() as u32 + 1, 30000000, m));
    // println!("{}", get_nth2(*inp.last().unwrap(), inp.len() as u32 + 1, 30, m));
}

fn get_nth2(last: u32, step: u32, max_step: u32, mem: &mut HashMap<u32, u32>) -> u32 {
    let mut last_num = last;
    for i in step..=max_step {
        // println!("last num: {}, mem {:?}", last_num, mem);
        match get_turn(mem, last_num) {
            0 => {
                mem.insert(last_num, i - 1);
                last_num = 0;
            }
            turn => {
                let new_num = i - 1 - turn;
                mem.insert(last_num, i - 1);
                last_num = new_num;
            }
        }
    }
    last_num
}

fn get_nth(last_num: u32, step: u32, max_step: u32, mem: &mut HashMap<u32, u32>) -> u32 {
    if step > max_step { return last_num; }
    match get_turn(mem, last_num) {
        0 => {
            mem.insert(last_num, step - 1);
            get_nth(0, step + 1, max_step, mem)
        }
        turn => {
            let new_num = step - 1 - turn;
            mem.insert(last_num, step - 1);
            get_nth(new_num, step + 1, max_step, mem)
        }
    }
}

fn get_turn(m: &HashMap<u32, u32>, val: u32) -> u32 {
    match m.get(&val) {
        Some(&turn) => turn,
        None => 0
    }
}

fn read_init(vals: &[u32]) -> HashMap<u32, u32> {
    vals.iter().enumerate().map(|(i, &val)| (val, i as u32 + 1)).collect()
}

fn input() -> Vec<u32> {
    // vec![0, 3, 6]
    // vec![3,1,2]
    vec![14, 1, 17, 0, 3, 20]
}
