pub fn part1() {
    let inp = parse_input();
    let min_time = inp.buses.iter().map(|(b, val)| (multiple_bus(*b, inp.target), b)).min().unwrap();
    println!("{:?}", (min_time.0 - inp.target) * min_time.1);
}

pub fn part2() {
    let i = parse_input2(&input());
    println!("{:?}", solve_part2(&i, 1, i[0]));
}

#[derive(Debug)]
struct BusSchedule {
    target: u64,
    buses: Vec<(u64, u64)>,
}

fn solve_part2(tups: &Vec<(u64, u64)>, curr: usize, res: (u64, u64)) -> (u64, u64) {
    if curr > tups.len() - 1 { return res; }
    solve_part2(tups, curr + 1, find_step(res.0, res.1, tups[curr]))
}

fn find_step(prev_val: u64, prev_step: u64, (ind, val): (u64, u64)) -> (u64, u64) {
    let mut result: u64 = prev_val;
    while (result + ind) % val != 0 {
        result += prev_step;
    }
    (result, prev_step * val)
}

fn multiple_bus(bus: u64, target: u64) -> u64 {
    let res: u64 = (target / bus) * bus;
    if res == target { res } else { res + bus }
}

fn parse_input2(s: &str) -> Vec<(u64, u64)> {
    let i: Vec<String> = s.split('\n').map(|s| s.to_string()).collect();
    i[1].split(',').enumerate().filter(|&(e, v)| v.ne("x")).map(|(e, v)| (e as u64, v.parse().unwrap())).collect()
}

fn parse_input() -> BusSchedule {
    let lines: Vec<String> = input().split('\n').map(|line| line.to_string()).collect();
    BusSchedule {
        target: lines[0].parse().unwrap(),
        buses: lines[1].split(',')
            .filter(|&x| x != "x")
            .map(|x| {
                let b: u64 = x.parse().unwrap();
                (b, b)
            }).collect(),
    }
}

fn input() -> String {
//     "939
// 7,13,x,x,59,x,31,19".to_string()

    "1007153
29,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,433,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,19,x,x,x,23,x,x,x,x,x,x,x,977,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,41".to_string()

//     "12
// 1789,37,47,1889".to_string()

//     "1
// 17,x,13,19".to_string()

//     "1
// 67,x,7,59,61".to_string()
}
